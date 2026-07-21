pub struct TaskInput {
    pub id: u32,
    pub input: u32,
}

impl TaskInput {
    pub fn new(id: u32, input: u32) -> Self {
        Self {
            id: id,
            input: input,
        }
    }
}

pub struct TaskResult {
    pub id: u32,
    pub result: u32,
}

impl TaskResult {
    pub fn new(id: u32, result: u32) -> Self {
        Self {
            id: id,
            result: result,
        }
    }
}

pub struct TimeOut {
    pub id: u32,
}

impl TimeOut {
    pub fn new(id: u32) -> Self {
        Self { id: id }
    }
}
