// Task related events between workers and orchestrator.
#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum Task {
    TaskResult {
        result: u32,
    },
    TaskTimeout {},
    TaskInput {
        input: u32,
    },
    /// A unit of multiplication work sent by an orchestrator to a worker.
    Multiply {
        a: u32,
        b: u32,
        k: usize,
    },
    /// A multiplication result sent by a worker back to its orchestrator.
    PartialResult {
        value: u32,
        k: usize,
    },
}
