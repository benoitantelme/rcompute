use crate::components::task::{TaskInput, TaskResult, TimeOut};

use std::time::SystemTime;

// Task related events between orchestrator and workers
pub enum TaskEvent {
    TaskFinished(TaskResult),
    TaskMissing(TimeOut),
    NewTask(TaskInput),
}

// Observability related events between orchestrator/workers and monitor
pub struct Event {
    id: u32,
    timestamp: SystemTime,
    source: Source,
    payload: EventPayload,
}

pub enum Source {
    Orchestrator,
    Worker(u32),
}

pub enum EventPayload {
    TaskAssigned { task_id: u32 },
    TaskStarted { task_id: u32 },
    TaskCompleted { task_id: u32 },
    TaskFailed { task_id: u32, reason: String },
}
