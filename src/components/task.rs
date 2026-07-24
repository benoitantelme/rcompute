// Task related events between workers and orchestrator
pub struct TaskEvent {
    pub worker_id: u32,
    pub task: Task,
}

impl TaskEvent {
    pub fn new(worker_id: u32, task: Task) -> Self {
        Self {
            worker_id: worker_id,
            task: task,
        }
    }
}

pub enum Task {
    TaskInput { id: u32, input: u32 },
    TaskResult { id: u32, result: u32 },
    TaskTimeout { id: u32, worker_id: u32 },
}