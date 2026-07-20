use crate::components::event::Event;
use crate::components::timer::Deadline;
use crate::components::worker::Worker;

use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::time::Duration;

pub struct Orchestrator {
    pub id: u32,
    initial_capacity: usize,
    pub threshold: u32,
    pub low_capacity: bool,
    pub empty: bool,
    workers: VecDeque<Worker>,
    pub busy_workers: HashSet<u32>,
    pub timeout: u64,
    pub check_frequency: u64,
    pub deadlines: Arc<Mutex<BinaryHeap<Deadline>>>,
    timeout_channel: (mpsc::Sender<u32>, mpsc::Receiver<u32>),
    events_channel: (mpsc::Sender<Event>, mpsc::Receiver<Event>),
}

impl Orchestrator {
    pub fn new(
        id: u32,
        initial_capacity: usize,
        threshold: u32,
        timeout: u64,
        check_frequency: u64,
    ) -> Self {
        Self {
            id: id,
            threshold: threshold,
            initial_capacity: initial_capacity,
            low_capacity: true,
            empty: true,
            workers: VecDeque::with_capacity(initial_capacity),
            busy_workers: HashSet::new(),
            timeout: timeout,
            check_frequency: check_frequency,
            deadlines: Arc::new(Mutex::new(BinaryHeap::new())),
            timeout_channel: mpsc::channel::<u32>(),
            events_channel: mpsc::channel::<Event>(),
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

        // listen for timeouts
        self.detect_timeouts();
        // just work once
        match self.timeout_channel.1.try_recv() {
            Ok(task_id) => {
                self.handle_timeout(task_id);
            }
            Err(mpsc::TryRecvError::Empty) => {
                // No timeout event right now, continue doing other work
                println!("No error")
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                // The sender has been dropped
                println!("Timeouts sender disconnected")
            }
        }

        loop {
            while let Ok(event) = self.events_channel.1.try_recv() {
                match event {
                    Event::Timeout(id) => self.handle_timeout(id),
                    Event::TaskFinished(result) => println!("self.handle_result(result)"),
                    Event::NewTask(task) => println!("self.add_task(task)"),
                }
            }

            // self.schedule();
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    fn detect_timeouts(&mut self) {
        let deadlines = Arc::clone(&self.deadlines);
        let sender = self.timeout_channel.0.clone();
        let check_frequency = self.check_frequency.clone();

        std::thread::spawn(move || {
            loop {
                std::thread::sleep(Duration::from_secs(check_frequency));

                let mut deadlines = deadlines.lock().unwrap();

                if deadlines.is_empty() {
                    break;
                }

                while let Some(deadline) = deadlines.peek() {
                    if deadline.is_expired() {
                        let expired = deadlines.pop().unwrap();
                        sender.send(expired.task_id).unwrap();
                        println!("Deadline reached for task {}", expired.task_id);
                    } else {
                        break;
                    }
                }
            }
        });
    }

    fn receive_timeouts(&mut self) {
        // no good can't share receivers need to pivot
        // let receiver = &self.timeout_channel.1;

        // std::thread::spawn(move || {
        //     loop {
        //         match receiver.recv() {
        //             Ok(task_id) => {
        //                 self.handle_timeout(task_id);
        //             }
        //             Err(mpsc::RecvError) => {
        //                 println!("Received error")
        //             }
        //         }
        //     }
        // });
    }

    pub fn push_worker(&mut self, worker: Worker) {
        // Managing timeouts
        self.deadlines
            .lock()
            .unwrap()
            .push(Deadline::new(worker.id, self.timeout));

        self.busy_workers.insert(worker.id);
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

        self.busy_workers.remove(&worker.id);

        println!("Pulling worker {}", worker.id);
        if self.workers.len() < self.threshold as usize {
            self.low_capacity = true;
        }

        worker
    }

    pub fn get_worker_queue_size(&mut self) -> usize {
        self.workers.len()
    }

    pub fn receive_result(&self, worker: Worker) -> (u32, u32, u32) {
        println!(
            "Received result from worker {} and task {}",
            worker.id, worker.task
        );
        (worker.id, worker.task, worker.calculate())
    }

    pub fn handle_timeout(&self, timer_id: u32) {
        println!("Received timeout for timer {} ", timer_id);

        //TODO: Handle timeout logic here, reset task, keep a trace of already failed task, loose worker ref?
    }
}

impl fmt::Display for Orchestrator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Orchestrator id {}", self.id)
    }
}
