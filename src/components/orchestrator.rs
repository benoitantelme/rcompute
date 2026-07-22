use crate::components::event::{EventPayload, MonitorEvent, Source, TaskEvent};
use crate::components::task::{TaskInput, TaskResult};
use crate::components::timer::Deadline;

use std::collections::BinaryHeap;
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
    pub timeout: u64,
    pub check_frequency: u64,
    pub deadlines: BinaryHeap<Deadline>,
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
            timeout: timeout,
            check_frequency: check_frequency,
            deadlines: BinaryHeap::new(),
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
            while let Ok(event) = self.task_events_receiver.try_recv() {
                match event {
                    TaskEvent::TaskMissing(timeout) => self.handle_timeout(timeout.id),
                    TaskEvent::TaskFinished(result) => self.handle_task_result(result),
                    TaskEvent::NewTask(task) => self.handle_task_creation(task),
                }
            }

            self.detect_timeouts();
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    pub fn push_worker(&mut self, worker_id: u32) {
        // Managing timeouts
        self.deadlines.push(Deadline::new(worker_id, self.timeout));

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

    pub fn receive_result(&self, worker_id: u32, task_result: u32) -> (u32, u32) {
        println!(
            "{} Received result from worker {} and task {}",
            ORCHESTRATOR, worker_id, task_result
        );
        (worker_id, task_result)
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
                self.handle_timeout(expired.task_id);
            } else {
                break;
            }
        }
    }

    pub fn handle_timeout(&self, task_id: u32) {
        println!("{} Received timeout for id {} ", ORCHESTRATOR, task_id);

        self.monitor_events_sender
            .send(MonitorEvent::new(
                self.id,
                SystemTime::now(),
                Source::Orchestrator,
                EventPayload::TaskFailed {
                    task_id: task_id,
                    reason: "Timeout".to_string(),
                },
            ))
            .unwrap();

        //TODO: Handle timeout logic here, reset task, keep a trace of already failed task, loose worker ref?
    }

    pub fn handle_task_result(&self, result: TaskResult) {
        println!(
            "{} Received result with id {} and content {}",
            ORCHESTRATOR, result.id, result.result
        );

        self.monitor_events_sender
            .send(MonitorEvent::new(
                self.id,
                SystemTime::now(),
                Source::Orchestrator,
                EventPayload::TaskCompleted { task_id: result.id },
            ))
            .unwrap();
    }

    pub fn handle_task_creation(&self, task: TaskInput) {
        println!(
            "{} Created task with id {} and input {}",
            ORCHESTRATOR, task.id, task.input
        );

        self.monitor_events_sender
            .send(MonitorEvent::new(
                self.id,
                SystemTime::now(),
                Source::Orchestrator,
                EventPayload::TaskStarted { task_id: task.id },
            ))
            .unwrap();
    }
}

impl fmt::Display for Orchestrator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} id {}", ORCHESTRATOR, self.id)
    }
}
