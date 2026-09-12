use crate::components::event::{EventPayload, MonitorEvent, Source};
use crate::components::task::{
    Task::{Multiply, PartialResult, TaskTimeout},
    TaskEvent,
};
use crate::config::app_config::AppConfig;

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::mpsc;
use std::time::{Duration, Instant, SystemTime};

const ORCHESTRATOR: &str = "Orchestrator: ";
const MATRIX_SIZE: usize = 3;

/// Coordinates real workers and records calculation state.
///
/// A worker exists only after its work channel is registered. There is no
/// synthetic worker pool: a SUMMA calculation uses workers 1 through 9 as a
/// row-major 3 by 3 grid of C matrix cells.
pub struct Orchestrator {
    pub id: u32,
    pub matrix_size: usize,
    pub workers: HashSet<u32>,
    pub busy_workers: HashSet<u32>,
    pub open_tasks: HashSet<u32>,
    pub closed_tasks: HashSet<u32>,
    pub failed_tasks: HashSet<u32>,
    /// Most recent scalar result for each SUMMA iteration k.
    pub partial_results: HashMap<usize, u32>,
    /// The matrix assembled by the most recent 3 by 3 SUMMA calculation.
    pub result_matrix: [[u32; MATRIX_SIZE]; MATRIX_SIZE],
    worker_task_senders: HashMap<u32, mpsc::Sender<TaskEvent>>,
    summa_assignments: HashMap<u32, (usize, usize)>,
    /// The worker and dispatch time for every multiplication still in flight.
    in_progress_tasks: HashMap<u32, (u32, Instant)>,
    task_timeout: Option<Duration>,
    timeout_check_frequency: Duration,
    task_events_receiver: mpsc::Receiver<TaskEvent>,
    monitor_events_sender: mpsc::Sender<MonitorEvent>,
}

impl Orchestrator {
    pub fn new(
        id: u32,
        monitor_events_sender: mpsc::Sender<MonitorEvent>,
        task_events_receiver: mpsc::Receiver<TaskEvent>,
        matrix_size: usize,
    ) -> Self {
        Self {
            id,
            matrix_size,
            workers: HashSet::new(),
            busy_workers: HashSet::new(),
            open_tasks: HashSet::new(),
            closed_tasks: HashSet::new(),
            failed_tasks: HashSet::new(),
            partial_results: HashMap::new(),
            result_matrix: [[0; MATRIX_SIZE]; MATRIX_SIZE],
            worker_task_senders: HashMap::new(),
            summa_assignments: HashMap::new(),
            in_progress_tasks: HashMap::new(),
            // `new` is retained for callers that do not supply configuration;
            // timeout enforcement is enabled by `from_config`.
            task_timeout: None,
            timeout_check_frequency: Duration::from_secs(1),
            task_events_receiver,
            monitor_events_sender,
        }
    }

    /// Creates an orchestrator configured for a matrix of the configured size.
    pub fn from_config(
        id: u32,
        monitor_events_sender: mpsc::Sender<MonitorEvent>,
        task_events_receiver: mpsc::Receiver<TaskEvent>,
        config: AppConfig,
    ) -> Self {
        let mut orchestrator = Self::new(
            id,
            monitor_events_sender,
            task_events_receiver,
            config.matrix_size,
        );
        
        orchestrator.task_timeout =
            (config.timeout != 0).then(|| Duration::from_millis(config.timeout));
        
        // A zero frequency is invalid for a periodic poll and would cause a
        // busy loop, so use a small safe interval in that case.
        orchestrator.timeout_check_frequency = Duration::from_millis(config.check_frequency.max(1));
        orchestrator
    }

    /// Registers a real worker and the channel on which it receives work.
    pub fn register_worker_channel(&mut self, worker_id: u32, sender: mpsc::Sender<TaskEvent>) {
        self.workers.insert(worker_id);
        self.worker_task_senders.insert(worker_id, sender);
    }

    /// Processes worker results until all worker result senders are dropped.
    pub fn run(mut self) {
        loop {
            match self
                .task_events_receiver
                .recv_timeout(self.timeout_check_frequency)
            {
                Ok(event) => {
                    let _ = self.handle_task_event(event);
                }
                Err(mpsc::RecvTimeoutError::Timeout) => self.check_task_timeouts(),
                Err(mpsc::RecvTimeoutError::Disconnected) => break,
            }
        }
    }

    /// Multiplies two 3 by 3 matrices using a 3 by 3 SUMMA worker grid.
    ///
    /// Workers 1 through 9 represent grid cells in row-major order: worker 1
    /// owns C[0][0], worker 2 owns C[0][1], and so on. For each `k`, the
    /// orchestrator sends A[i][k] * B[k][j] to the worker that owns C[i][j].
    pub fn multiply_summa(
        &mut self,
        a: [[u32; MATRIX_SIZE]; MATRIX_SIZE],
        b: [[u32; MATRIX_SIZE]; MATRIX_SIZE],
    ) -> Result<[[u32; MATRIX_SIZE]; MATRIX_SIZE], String> {
        if self.matrix_size != MATRIX_SIZE {
            return Err(format!(
                "This SUMMA implementation supports matrix_size {MATRIX_SIZE}, got {}",
                self.matrix_size
            ));
        }

        self.ensure_summa_grid()?;
        self.result_matrix = [[0; MATRIX_SIZE]; MATRIX_SIZE];
        self.summa_assignments.clear();

        for k in 0..MATRIX_SIZE {
            for i in 0..MATRIX_SIZE {
                for j in 0..MATRIX_SIZE {
                    let worker_id = (i * MATRIX_SIZE + j + 1) as u32;
                    let task_id = (k * MATRIX_SIZE * MATRIX_SIZE + i * MATRIX_SIZE + j + 1) as u32;
                    self.dispatch_multiply_to_worker(worker_id, task_id, a[i][k], b[k][j], k)?;
                    self.summa_assignments.insert(task_id, (i, j));
                }
            }

            while self.open_tasks.len() > 0 {
                match self
                    .task_events_receiver
                    .recv_timeout(self.timeout_check_frequency)
                {
                    Ok(event) => self.handle_task_event(event)?,
                    Err(mpsc::RecvTimeoutError::Timeout) => self.check_task_timeouts(),
                    Err(mpsc::RecvTimeoutError::Disconnected) => {
                        return Err(format!(
                            "Worker results disconnected during SUMMA iteration {k}"
                        ));
                    }
                }

                if self.failed_tasks.len() > 0 {
                    return Err(format!(
                        "SUMMA iteration {k} has failed multiplication tasks"
                    ));
                }
            }
        }

        Ok(self.result_matrix)
    }

    fn ensure_summa_grid(&self) -> Result<(), String> {
        for worker_id in 1..=(MATRIX_SIZE * MATRIX_SIZE) as u32 {
            if !self.workers.contains(&worker_id) {
                return Err(format!("SUMMA worker {worker_id} is not registered"));
            }
        }
        Ok(())
    }

    fn dispatch_multiply_to_worker(
        &mut self,
        worker_id: u32,
        task_id: u32,
        a: u32,
        b: u32,
        k: usize,
    ) -> Result<(), String> {
        if !self.workers.contains(&worker_id) {
            return Err(format!("Worker {worker_id} is not registered"));
        }
        if !self.busy_workers.insert(worker_id) {
            return Err(format!("Worker {worker_id} is already busy"));
        }

        let Some(sender) = self.worker_task_senders.get(&worker_id) else {
            self.busy_workers.remove(&worker_id);
            return Err(format!("Worker {worker_id} has no registered work channel"));
        };

        if sender
            .send(TaskEvent::new(worker_id, task_id, Multiply { a, b, k }))
            .is_err()
        {
            self.busy_workers.remove(&worker_id);
            return Err(format!("Worker {worker_id} work channel is disconnected"));
        }

        self.open_tasks.insert(task_id);
        self.in_progress_tasks
            .insert(task_id, (worker_id, Instant::now()));
        
        self.monitor_events_sender
            .send(MonitorEvent::new(
                self.id,
                SystemTime::now(),
                Source::Orchestrator,
                EventPayload::TaskAssigned { task_id, worker_id },
            ))
            .unwrap();
        Ok(())
    }

    pub fn handle_timeout(&mut self, task_event: TaskEvent) {
        if !self.open_tasks.remove(&task_event.task_id) {
            return;
        }
        self.monitor_events_sender
            .send(MonitorEvent::new(
                self.id,
                SystemTime::now(),
                Source::Orchestrator,
                EventPayload::TaskFailed {
                    task_id: task_event.task_id,
                    worker_id: task_event.worker_id,
                    reason: "Timeout".to_string(),
                },
            ))
            .unwrap();

        self.in_progress_tasks.remove(&task_event.task_id);
        self.failed_tasks.insert(task_event.task_id);
        self.busy_workers.remove(&task_event.worker_id);
    }

    pub fn handle_partial_result(&mut self, task_id: u32, worker_id: u32, value: u32, k: usize) {
        // Don't process results that arrive after the timeout 
        if !self.open_tasks.remove(&task_id) {
            return;
        }

        self.partial_results.insert(k, value);
        if let Some((i, j)) = self.summa_assignments.remove(&task_id) {
            self.result_matrix[i][j] += value;
        }

        self.in_progress_tasks.remove(&task_id);
        self.closed_tasks.insert(task_id);
        self.busy_workers.remove(&worker_id);

        self.monitor_events_sender
            .send(MonitorEvent::new(
                self.id,
                SystemTime::now(),
                Source::Orchestrator,
                EventPayload::TaskCompleted { task_id, worker_id },
            ))
            .unwrap();
    }

    /// Process available results
    pub fn process_incoming_tasks(&mut self) {
        while let Ok(event) = self.task_events_receiver.try_recv() {
            let _ = self.handle_task_event(event);
        }
        self.check_task_timeouts();
    }

    pub fn check_task_timeouts(&mut self) {
        let Some(timeout) = self.task_timeout else {
            return;
        };

        let expired_tasks: Vec<_> = self
            .in_progress_tasks
            .iter()
            .filter_map(|(&task_id, &(worker_id, started_at))| {
                (started_at.elapsed() >= timeout).then_some((task_id, worker_id))
            })
            .collect();

        for (task_id, worker_id) in expired_tasks {
            self.handle_timeout(TaskEvent::new(worker_id, task_id, TaskTimeout {}));
        }
    }

    fn handle_task_event(&mut self, event: TaskEvent) -> Result<(), String> {
        match event.task {
            TaskTimeout {} => {
                let worker_id = event.worker_id;
                self.handle_timeout(event);
                Err(format!("Worker {worker_id} timed out"))
            }
            PartialResult { value, k } => {
                self.handle_partial_result(event.task_id, event.worker_id, value, k);
                Ok(())
            }
            Multiply { .. } => Err("Orchestrator received a Multiply task".to_string()),
        }
    }
}

impl fmt::Display for Orchestrator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} id {}", ORCHESTRATOR, self.id)
    }
}
