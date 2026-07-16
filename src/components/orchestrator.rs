use crate::components::worker::Worker;
use std::collections::VecDeque;
use std::fmt;

pub struct Orchestrator {
    pub id: u32,
    pub workers: VecDeque<Worker>,
}

impl Orchestrator {
    pub fn new(id: u32, capacity: usize) -> Self {
        Self {
            id: id,
            workers: VecDeque::with_capacity(capacity),
        }
    }

    pub fn receive_result(&self, worker: Worker) -> (u32, u32, u32) {
        println!(
            "Received result from worker {} and task {}",
            worker.id, worker.task
        );
        (worker.id, worker.task, worker.calculate())
    }
}

impl fmt::Display for Orchestrator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Orchestrator id {}", self.id)
    }
}
