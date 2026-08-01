use crate::components::event::EventPayload;
use crate::components::event::MonitorEvent;
use crate::components::event::Source;
use crate::components::task::Task::TaskResult;
use crate::components::task::Task::TaskTimeout;
use crate::components::task::TaskEvent;

use std::fmt;
use std::sync::mpsc;
use std::time::SystemTime;

const WORKER: &str = "Worker: ";

pub struct Worker {
    pub id: u32,
    pub task: u32,
    tasks_events_sender: mpsc::Sender<TaskEvent>,
    monitor_events_sender: mpsc::Sender<MonitorEvent>,
}

impl Worker {
    pub fn new(
        id: u32,
        task: u32,
        t_sender: mpsc::Sender<TaskEvent>,
        m_sender: mpsc::Sender<MonitorEvent>,
    ) -> Self {
        Self {
            id: id,
            task: task,
            tasks_events_sender: t_sender,
            monitor_events_sender: m_sender,
        }
    }

    pub fn calculate(&self) -> u32 {
        println!("{} id {} is calculating", WORKER, self.id);

        self.monitor_events_sender
            .send(MonitorEvent::new(
                self.id,
                SystemTime::now(),
                Source::Worker(self.id),
                EventPayload::TaskCompleted {
                    task_id: self.task,
                    worker_id: self.id,
                },
            ))
            .unwrap();

        self.tasks_events_sender
            .send(TaskEvent::new(
                self.id,
                self.task,
                TaskResult { result: 42 },
            ))
            .unwrap();

        return 42;
    }

    pub fn timeout(&self) -> u32 {
        println!("{} id {} has timed out", WORKER, self.id);

        self.monitor_events_sender
            .send(MonitorEvent::new(
                self.id,
                SystemTime::now(),
                Source::Worker(self.id),
                EventPayload::TaskFailed {
                    task_id: self.task,
                    worker_id: self.id,
                    reason: "Timeout".to_string(),
                },
            ))
            .unwrap();

        self.tasks_events_sender
            .send(TaskEvent::new(self.id, self.task, TaskTimeout {}))
            .unwrap();

        return 42;
    }
}

impl fmt::Display for Worker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} id {}", WORKER, self.id)
    }
}
