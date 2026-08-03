use std::time::SystemTime;

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

#[derive(Clone, Debug, PartialEq)]
pub enum Source {
    Orchestrator,
    Worker(u32),
}

#[derive(Clone, Debug, PartialEq)]
pub enum EventPayload {
    TaskAssigned {
        task_id: u32,
        worker_id: u32,
    },
    TaskStarted {
        task_id: u32,
        worker_id: u32,
    },
    TaskCompleted {
        task_id: u32,
        worker_id: u32,
    },
    TaskFailed {
        task_id: u32,
        worker_id: u32,
        reason: String,
    },
    TaskDuplicated {
        task_id: u32,
        worker_id: u32,
    },
    TaskOrdered {
        task_id: u32,
        worker_id: u32,
    },
}
