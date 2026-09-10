use crate::components::event::{EventPayload, MonitorEvent, Source};
use crate::components::task::Task;
use crate::components::task::{
    Task::{Multiply, PartialResult, TaskTimeout},
    TaskEvent,
};
use crate::components::timer::Deadline;
use crate::config::app_config::AppConfig;

use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::sync::mpsc;
use std::time::{Duration, SystemTime};
const ORCHESTRATOR: &str = "Orchestrator: ";

pub struct Orchestrator {
    pub id: u32,
    initial_capacity: usize,
    pub threshold: u32,
    pub low_capacity: bool,
    pub empty: bool,
    pub available_workers: VecDeque<u32>,
    pub busy_workers: HashSet<u32>,
    pub open_tasks: HashSet<u32>,
    pub closed_tasks: HashSet<u32>,
    pub failed_tasks: HashSet<u32>,
    pub timeout: u64,
    pub check_frequency: u64,
    pub deadlines: BinaryHeap<Deadline>,
    /// Senders for the per-worker work channels.
    worker_task_senders: HashMap<u32, mpsc::Sender<TaskEvent>>,
    /// Partial multiplication results indexed by their `k` coordinate.
    pub partial_results: HashMap<usize, u32>,
    /// The matrix assembled by the most recent 3 by 3 SUMMA calculation.
    pub result_matrix: [[u32; 3]; 3],
    summa_assignments: HashMap<u32, (usize, usize)>,
    task_events_receiver: mpsc::Receiver<TaskEvent>,
    monitor_events_sender: mpsc::Sender<MonitorEvent>,
}

impl Orchestrator {
    pub fn new(
        id: u32,
        monitor_events_sender: mpsc::Sender<MonitorEvent>,
        task_events_receiver: mpsc::Receiver<TaskEvent>,
        initial_capacity: usize,
        threshold: u32,
        timeout: u64,
        check_frequency: u64,
    ) -> Self {
        Self {
            id: id,
            monitor_events_sender: monitor_events_sender,
            task_events_receiver: task_events_receiver,
            threshold: threshold,
            initial_capacity: initial_capacity,
            low_capacity: true,
            empty: true,
            available_workers: VecDeque::with_capacity(initial_capacity),
            busy_workers: HashSet::new(),
            open_tasks: HashSet::new(),
            closed_tasks: HashSet::new(),
            failed_tasks: HashSet::new(),
            timeout: timeout,
            check_frequency: check_frequency,
            deadlines: BinaryHeap::new(),
            worker_task_senders: HashMap::new(),
            partial_results: HashMap::new(),
            result_matrix: [[0; 3]; 3],
            summa_assignments: HashMap::new(),
        }
    }

    pub fn from_config(
        id: u32,
        monitor_events_sender: mpsc::Sender<MonitorEvent>,
        task_events_receiver: mpsc::Receiver<TaskEvent>,
        config: AppConfig,
    ) -> Self {
        Self {
            id: id,
            monitor_events_sender: monitor_events_sender,
            task_events_receiver: task_events_receiver,
            threshold: config.workers_threshold,
            initial_capacity: config.workers_number,
            low_capacity: true,
            empty: true,
            available_workers: VecDeque::with_capacity(config.workers_number),
            busy_workers: HashSet::new(),
            open_tasks: HashSet::new(),
            closed_tasks: HashSet::new(),
            failed_tasks: HashSet::new(),
            timeout: config.timeout,
            check_frequency: config.check_frequency,
            deadlines: BinaryHeap::new(),
            worker_task_senders: HashMap::new(),
            partial_results: HashMap::new(),
            result_matrix: [[0; 3]; 3],
            summa_assignments: HashMap::new(),
        }
    }

    pub fn initialise(&mut self) {
        for n in 1..self.initial_capacity as u32 + 1 {
            self.push_worker(n);
        }

        println!(
            "{} {} initialised with {} workers",
            ORCHESTRATOR,
            self.id,
            self.available_workers.len()
        );
    }

    pub fn run(mut self) {
        loop {
            self.process_incoming_tasks();

            // TODO: Send out new calculations, received via orders? later

            self.detect_timeouts();
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    pub fn push_worker(&mut self, worker_id: u32) {
        // Managing timeouts
        // TODO: need to manage task separately later
        self.deadlines
            .push(Deadline::new(42, worker_id, self.timeout));

        println!("{} Adding worker {}", ORCHESTRATOR, worker_id);
        self.available_workers.push_back(worker_id);

        if self.available_workers.len() >= self.threshold as usize {
            self.low_capacity = false;
        }
        self.empty = false;
    }

    pub fn pull_worker(&mut self) -> u32 {
        let wrapped_worker = self.available_workers.pop_front();
        let worker_id;
        match wrapped_worker {
            Some(value) => worker_id = value,
            None => panic!("{} No workers available", ORCHESTRATOR),
        }

        self.busy_workers.remove(&worker_id);

        println!("{} Pulling worker {}", ORCHESTRATOR, worker_id);
        if self.available_workers.len() < self.threshold as usize {
            self.low_capacity = true;
        }

        worker_id
    }

    pub fn get_worker_queue_size(&mut self) -> usize {
        self.available_workers.len()
    }

    /// Registers a worker's inbound work channel with this orchestrator.
    pub fn register_worker_channel(&mut self, worker_id: u32, sender: mpsc::Sender<TaskEvent>) {
        self.worker_task_senders.insert(worker_id, sender);
    }

    /// Assigns a multiplication task to the next available registered worker.
    /// Returns the selected worker id on success.
    pub fn dispatch_multiply(
        &mut self,
        task_id: u32,
        a: u32,
        b: u32,
        k: usize,
    ) -> Result<u32, String> {
        if self.open_tasks.contains(&task_id) {
            return Err(format!("Task {} is already open", task_id));
        }

        let worker_id = self.pull_worker();
        let Some(sender) = self.worker_task_senders.get(&worker_id) else {
            self.push_worker(worker_id);
            return Err(format!(
                "Worker {} has no registered work channel",
                worker_id
            ));
        };

        if sender
            .send(TaskEvent::new(worker_id, task_id, Multiply { a, b, k }))
            .is_err()
        {
            self.push_worker(worker_id);
            return Err(format!("Worker {} work channel is disconnected", worker_id));
        }

        self.busy_workers.insert(worker_id);
        self.open_tasks.insert(task_id);
        self.monitor_events_sender
            .send(MonitorEvent::new(
                self.id,
                SystemTime::now(),
                Source::Orchestrator,
                EventPayload::TaskAssigned { task_id, worker_id },
            ))
            .unwrap();
        Ok(worker_id)
    }

    /// Multiplies two 3 by 3 matrices using a 3 by 3 SUMMA worker grid.
    ///
    /// Workers 1 through 9 represent grid cells in row-major order: worker 1
    /// owns C[0][0], worker 2 owns C[0][1], and so on. For each `k`, the
    /// orchestrator sends A[i][k] * B[k][j] to the worker that owns C[i][j].
    pub fn multiply_summa(
        &mut self,
        a: [[u32; 3]; 3],
        b: [[u32; 3]; 3],
    ) -> Result<[[u32; 3]; 3], String> {
        self.result_matrix = [[0; 3]; 3];
        self.summa_assignments.clear();

        for k in 0..3 {
            for i in 0..3 {
                for j in 0..3 {
                    let worker_id = (i * 3 + j + 1) as u32;
                    let task_id = (k * 9 + i * 3 + j + 1) as u32;
                    self.dispatch_multiply_to_worker(worker_id, task_id, a[i][k], b[k][j], k)?;
                    self.summa_assignments.insert(task_id, (i, j));
                }
            }

            for _ in 0..9 {
                let event = self
                    .task_events_receiver
                    .recv_timeout(Duration::from_secs(1))
                    .map_err(|_| format!("Timed out waiting for SUMMA iteration {k} results"))?;
                self.handle_task_event(event)?;
            }
        }

        Ok(self.result_matrix)
    }

    fn dispatch_multiply_to_worker(
        &mut self,
        worker_id: u32,
        task_id: u32,
        a: u32,
        b: u32,
        k: usize,
    ) -> Result<(), String> {
        let Some(position) = self
            .available_workers
            .iter()
            .position(|available_id| *available_id == worker_id)
        else {
            return Err(format!("SUMMA worker {worker_id} is not available"));
        };
        self.available_workers.remove(position);

        let Some(sender) = self.worker_task_senders.get(&worker_id) else {
            self.push_worker(worker_id);
            return Err(format!(
                "SUMMA worker {worker_id} has no registered work channel"
            ));
        };

        if sender
            .send(TaskEvent::new(worker_id, task_id, Multiply { a, b, k }))
            .is_err()
        {
            self.push_worker(worker_id);
            return Err(format!(
                "SUMMA worker {worker_id} work channel is disconnected"
            ));
        }

        self.busy_workers.insert(worker_id);
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

    // TODO: see if possible to return last non achieved timeout so we can sleep for that duration
    fn detect_timeouts(&mut self) {
        if self.deadlines.is_empty() {
            return;
        }

        while let Some(deadline) = self.deadlines.peek() {
            if deadline.is_expired() {
                let expired = self.deadlines.pop().unwrap();
                println!(
                    "{} Deadline reached for task {}",
                    ORCHESTRATOR, expired.task_id
                );
                self.handle_timeout(TaskEvent::new(
                    expired.worker_id,
                    expired.task_id,
                    Task::TaskTimeout {},
                ));
            } else {
                break;
            }
        }
    }

    pub fn handle_timeout(&mut self, task_event: TaskEvent) {
        println!(
            "{} Received timeout for task {} from worker {}",
            ORCHESTRATOR, task_event.task_id, task_event.worker_id
        );

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

        // TODO: need to manage retry later?
        self.open_tasks.remove(&task_event.task_id);
        self.failed_tasks.insert(task_event.task_id);

        match self.busy_workers.remove(&task_event.worker_id) {
            true => {
                println!(
                    "{} Timeout for worker {} while it is still busy, removing from busy list",
                    ORCHESTRATOR, task_event.worker_id
                );
            }
            _ => {}
        };
    }

    pub fn handle_partial_result(&mut self, task_id: u32, worker_id: u32, value: u32, k: usize) {
        self.partial_results.insert(k, value);
        if let Some((i, j)) = self.summa_assignments.remove(&task_id) {
            self.result_matrix[i][j] += value;
        }
        self.open_tasks.remove(&task_id);
        self.closed_tasks.insert(task_id);

        self.monitor_events_sender
            .send(MonitorEvent::new(
                self.id,
                SystemTime::now(),
                Source::Orchestrator,
                EventPayload::TaskCompleted { task_id, worker_id },
            ))
            .unwrap();

        if self.busy_workers.remove(&worker_id) {
            self.push_worker(worker_id);
        }
    }

    /// Drains results sent by workers. Kept public so callers that own the
    /// event loop can drive the orchestrator without starting `run`.
    pub fn process_incoming_tasks(&mut self) {
        while let Ok(event) = self.task_events_receiver.try_recv() {
            // A non-SUMMA event cannot fail this non-blocking maintenance path.
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
