use crate::components::worker::Worker;
use std::collections::VecDeque;
use std::fmt;

pub struct Orchestrator {
    pub id: u32,
    initial_capacity: usize,
    pub threshold: u32,
    pub low_capacity: bool,
    pub empty: bool,
    workers: VecDeque<Worker>,
}

impl Orchestrator {
    pub fn new(id: u32, initial_capacity: usize, threshold: u32) -> Self {
        Self {
            id: id,
            threshold: threshold,
            initial_capacity: initial_capacity,
            low_capacity: true,
            empty: true,
            workers: VecDeque::with_capacity(initial_capacity),
        }
    }

    pub fn initialise(&mut self) {
        for n in 1..self.initial_capacity as u32 + 1 {
            let worker = Worker::new(n, n);
            self.push_worker(worker);
        }

        println!(
            "Orchestrator {} initialised with {} workers",
            self.id,
            self.workers.len()
        );
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

        if self.workers.len() >= self.threshold as usize {
            self.low_capacity = false;
        }
        self.empty = false;
    }

    pub fn pull_worker(&mut self) -> Worker {
        let wrapped_worker = self.workers.pop_front();
        let worker;
        match wrapped_worker {
            Some(value) => worker = value,
            None => panic!("No workers available"),
        }

        println!("Pulling worker {}", worker.id);
        if self.workers.len() < self.threshold as usize {
            self.low_capacity = true;
        }

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
