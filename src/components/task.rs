// Task related events between workers and orchestrator
pub struct TaskEvent {
    pub worker_id: u32,
    pub task_id: u32,
    pub task: Task,
}

impl TaskEvent {
    pub fn new(worker_id: u32, task_id: u32, task: Task) -> Self {
        Self {
            worker_id: worker_id,
            task_id: task_id,
            task: task,
        }
    }
}

pub enum Task {
    TaskResult { result: u32 },
    TaskTimeout {},
    TaskInput { input: u32 },
}
