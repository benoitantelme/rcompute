use crate::components::event::Event;
use crate::components::task::TaskResult;

use std::fmt;
use std::sync::mpsc;

pub struct Worker {
    pub id: u32,
    pub task: u32,
    events_sender: mpsc::Sender<Event>,
}

impl Worker {
    pub fn new(id: u32, task: u32, sender: mpsc::Sender<Event>) -> Self {
        Self {
            id: id,
            task: task,
            events_sender: sender,
        }
    }

    pub fn calculate(&self) -> u32 {
        println!("Worker id {} is calculating", self.id);
        self.events_sender
            .send(Event::TaskFinished(TaskResult::new(self.task, 42)))
            .unwrap();
        return 42;
    }
}

impl fmt::Display for Worker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Worker id {}", self.id)
    }
}
