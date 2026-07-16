use crate::components::orchestrator::Orchestrator;
use std::fmt;

pub struct Worker {
    pub id: u32,
    pub task: u32,
    pub orchestrator: Orchestrator,
}

impl Worker {
    pub fn new(id: u32, task: u32, orchestrator: Orchestrator) -> Self {
        Self {
            id: id,
            task: task,
            orchestrator: orchestrator,
        }
    }
}

impl fmt::Display for Worker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Worker id {}", self.id)
    }
}
