use crate::components::task::{TaskInput, TaskResult, TimeOut};

use std::time::SystemTime;

// Task related events between orchestrator and workers
pub enum TaskEvent {
    TaskFinished(TaskResult),
    TaskMissing(TimeOut),
    NewTask(TaskInput),
}

// Observability related events between orchestrator/workers and monitor
#[derive(Clone)]
pub struct MonitorEvent {
    pub id: u32,
    pub timestamp: SystemTime,
    pub source: Source,
    pub payload: EventPayload,
}

impl MonitorEvent {
    pub fn new(id: u32, timestamp: SystemTime, source: Source, payload: EventPayload) -> Self {
        Self {
            id: id,
            timestamp: timestamp,
            source: source,
            payload: payload,
        }
    }
}

#[derive(Clone)]
pub enum Source {
    Orchestrator,
    Worker(u32),
}

#[derive(Clone)]
pub enum EventPayload {
    TaskAssigned { task_id: u32 },
    TaskStarted { task_id: u32 },
    TaskCompleted { task_id: u32 },
    TaskFailed { task_id: u32, reason: String },
}
