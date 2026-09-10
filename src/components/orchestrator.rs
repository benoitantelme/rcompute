use crate::components::event::{EventPayload, MonitorEvent, Source};
use crate::components::task::{
    Task::{Multiply, PartialResult, TaskTimeout},
    TaskEvent,
};
use crate::config::app_config::AppConfig;

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::mpsc;
use std::time::{Duration, SystemTime};

const ORCHESTRATOR: &str = "Orchestrator: ";
const SUMMA_DIMENSION: usize = 3;

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
    pub result_matrix: [[u32; SUMMA_DIMENSION]; SUMMA_DIMENSION],
    worker_task_senders: HashMap<u32, mpsc::Sender<TaskEvent>>,
    summa_assignments: HashMap<u32, (usize, usize)>,
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
            result_matrix: [[0; SUMMA_DIMENSION]; SUMMA_DIMENSION],
            worker_task_senders: HashMap::new(),
            summa_assignments: HashMap::new(),
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
        Self::new(
            id,
            monitor_events_sender,
            task_events_receiver,
            config.matrix_size,
        )
    }

    /// Registers a real worker and the channel on which it receives work.
    pub fn register_worker_channel(&mut self, worker_id: u32, sender: mpsc::Sender<TaskEvent>) {
        self.workers.insert(worker_id);
        self.worker_task_senders.insert(worker_id, sender);
    }

    /// Processes worker results until all worker result senders are dropped.
    pub fn run(mut self) {
        while let Ok(event) = self.task_events_receiver.recv() {
            let _ = self.handle_task_event(event);
        }
    }

    /// Multiplies two 3 by 3 matrices using a 3 by 3 SUMMA worker grid.
    ///
    /// Workers 1 through 9 represent grid cells in row-major order: worker 1
    /// owns C[0][0], worker 2 owns C[0][1], and so on. For each `k`, the
    /// orchestrator sends A[i][k] * B[k][j] to the worker that owns C[i][j].
    pub fn multiply_summa(
        &mut self,
        a: [[u32; SUMMA_DIMENSION]; SUMMA_DIMENSION],
        b: [[u32; SUMMA_DIMENSION]; SUMMA_DIMENSION],
    ) -> Result<[[u32; SUMMA_DIMENSION]; SUMMA_DIMENSION], String> {
        if self.matrix_size != SUMMA_DIMENSION {
            return Err(format!(
                "This SUMMA implementation supports matrix_size {SUMMA_DIMENSION}, got {}",
                self.matrix_size
            ));
        }
        self.ensure_summa_grid()?;
        self.result_matrix = [[0; SUMMA_DIMENSION]; SUMMA_DIMENSION];
        self.summa_assignments.clear();

        for k in 0..SUMMA_DIMENSION {
            for i in 0..SUMMA_DIMENSION {
                for j in 0..SUMMA_DIMENSION {
                    let worker_id = (i * SUMMA_DIMENSION + j + 1) as u32;
                    let task_id =
                        (k * SUMMA_DIMENSION * SUMMA_DIMENSION + i * SUMMA_DIMENSION + j + 1)
                            as u32;
                    self.dispatch_multiply_to_worker(worker_id, task_id, a[i][k], b[k][j], k)?;
                    self.summa_assignments.insert(task_id, (i, j));
                }
            }

            for _ in 0..SUMMA_DIMENSION * SUMMA_DIMENSION {
                let event = self
                    .task_events_receiver
                    .recv_timeout(Duration::from_secs(1))
                    .map_err(|_| format!("Timed out waiting for SUMMA iteration {k} results"))?;
                self.handle_task_event(event)?;
            }
        }

        Ok(self.result_matrix)
    }

    fn ensure_summa_grid(&self) -> Result<(), String> {
        for worker_id in 1..=(SUMMA_DIMENSION * SUMMA_DIMENSION) as u32 {
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
        self.open_tasks.remove(&task_event.task_id);
        self.failed_tasks.insert(task_event.task_id);
        self.busy_workers.remove(&task_event.worker_id);
    }

    pub fn handle_partial_result(&mut self, task_id: u32, worker_id: u32, value: u32, k: usize) {
        self.partial_results.insert(k, value);
        if let Some((i, j)) = self.summa_assignments.remove(&task_id) {
            self.result_matrix[i][j] += value;
        }
        self.open_tasks.remove(&task_id);
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

    /// Drains currently available worker results without blocking.
    pub fn process_incoming_tasks(&mut self) {
        while let Ok(event) = self.task_events_receiver.try_recv() {
            let _ = self.handle_task_event(event);
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
