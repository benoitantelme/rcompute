use std::collections::VecDeque;
use std::fmt;

pub struct Orchestrator {
    pub id: u32,
    pub workers: VecDeque<i32>,
}

impl Orchestrator {
    pub fn new(id: u32) -> Self {
        Self {
            id: id,
            workers: VecDeque::with_capacity(10),
        }
    }
}

impl fmt::Display for Orchestrator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Orchestrator id {}", self.id)
    }
}
