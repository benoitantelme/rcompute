use std::fmt;
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

impl fmt::Display for MonitorEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Event {} with source {} ", self.id, self.source)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Source {
    Orchestrator,
    Worker(u32),
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Source::Orchestrator => write!(f, "Orchestrator"),
            Source::Worker(id) => write!(f, "Worker {}", id),
        }
    }
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
