use crate::components::worker::Worker;
use std::collections::VecDeque;
use std::fmt;

pub struct Orchestrator {
    pub id: u32,
    workers: VecDeque<Worker>,
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

    pub fn push_worker(&mut self, worker: Worker) {
        println!("Adding worker {}", worker.id);
        self.workers.push_back(worker);
    }

    pub fn pull_worker(&mut self) -> Worker {
        let worker = self.workers.pop_front().unwrap();
        println!("Pulling worker {}", worker.id);

        worker
    }

    pub fn get_worker_queue_size(&mut self) -> usize {
        self.workers.len()
    }
}

impl fmt::Display for Orchestrator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Orchestrator id {}", self.id)
    }
}
