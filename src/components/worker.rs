use std::fmt;

pub struct Worker {
    pub id: u32,
    pub task: u32,
}

impl Worker {
    pub fn new(id: u32, task: u32) -> Self {
        Self { id: id, task: task }
    }

    pub fn calculate(&self) -> u32 {
        println!("Worker id {} is calculating", self.id);
        return 42;
    }
}

impl fmt::Display for Worker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Worker id {}", self.id)
    }
}
