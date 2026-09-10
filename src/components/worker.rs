use crate::components::event::EventPayload;
use crate::components::event::MonitorEvent;
use crate::components::event::Source;
use crate::components::task::Task::Multiply;
use crate::components::task::Task::PartialResult;
use crate::components::task::Task::TaskInput;
use crate::components::task::Task::TaskResult;
use crate::components::task::Task::TaskTimeout;
use crate::components::task::TaskEvent;

use std::fmt;
use std::sync::mpsc;
use std::time::SystemTime;

const WORKER: &str = "Worker: ";

pub struct Worker {
    pub id: u32,
    tasks_events_sender: mpsc::Sender<TaskEvent>,
    monitor_events_sender: mpsc::Sender<MonitorEvent>,
    work_receiver: Option<mpsc::Receiver<TaskEvent>>,
}

impl Worker {
    pub fn new(
        id: u32,
        t_sender: mpsc::Sender<TaskEvent>,
        m_sender: mpsc::Sender<MonitorEvent>,
    ) -> Self {
        Self {
            id: id,
            tasks_events_sender: t_sender,
            monitor_events_sender: m_sender,
            work_receiver: None,
        }
    }

    /// Creates a worker with its incoming work channel and returns the sender
    /// that must be registered with the orchestrator.
    pub fn with_work_channel(
        id: u32,
        result_sender: mpsc::Sender<TaskEvent>,
        monitor_sender: mpsc::Sender<MonitorEvent>,
    ) -> (Self, mpsc::Sender<TaskEvent>) {
        let (work_sender, work_receiver) = mpsc::channel();
        (
            Self {
                id,
                tasks_events_sender: result_sender,
                monitor_events_sender: monitor_sender,
                work_receiver: Some(work_receiver),
            },
            work_sender,
        )
    }

    /// Listens for work from the orchestrator until its work channel closes.
    pub fn run(self) {
        let Some(ref work_receiver) = self.work_receiver else {
            return;
        };

        while let Ok(task_event) = work_receiver.recv() {
            self.handle_task(task_event);
        }
    }

    fn handle_task(&self, task_event: TaskEvent) {
        match task_event.task {
            Multiply { a, b, k } => {
                let value = a * b;

                self.monitor_events_sender
                    .send(MonitorEvent::new(
                        self.id,
                        SystemTime::now(),
                        Source::Worker(self.id),
                        EventPayload::TaskCompleted {
                            task_id: task_event.task_id,
                            worker_id: self.id,
                        },
                    ))
                    .unwrap();

                self.tasks_events_sender
                    .send(TaskEvent::new(
                        self.id,
                        task_event.task_id,
                        PartialResult { value, k },
                    ))
                    .unwrap();
            }
            _ => {}
        }
    }

    pub fn calculate(&self, task_id: u32) -> u32 {
        println!("{} id {} is calculating", WORKER, self.id);

        self.monitor_events_sender
            .send(MonitorEvent::new(
                self.id,
                SystemTime::now(),
                Source::Worker(self.id),
                EventPayload::TaskCompleted {
                    task_id: task_id,
                    worker_id: self.id,
                },
            ))
            .unwrap();

        self.tasks_events_sender
            .send(TaskEvent::new(self.id, task_id, TaskResult { result: 42 }))
            .unwrap();

        return 42;
    }

    pub fn timeout(&self, task_id: u32) -> u32 {
        println!("{} id {} has timed out", WORKER, self.id);

        self.monitor_events_sender
            .send(MonitorEvent::new(
                self.id,
                SystemTime::now(),
                Source::Worker(self.id),
                EventPayload::TaskFailed {
                    task_id: task_id,
                    worker_id: self.id,
                    reason: "Timeout".to_string(),
                },
            ))
            .unwrap();

        self.tasks_events_sender
            .send(TaskEvent::new(self.id, task_id, TaskTimeout {}))
            .unwrap();

        return 42;
    }

    pub fn send_task(&self, task_id: u32, input: u32) -> u32 {
        println!(
            "{} id {} sending task  {} input {}",
            WORKER, self.id, task_id, input
        );

        self.monitor_events_sender
            .send(MonitorEvent::new(
                self.id,
                SystemTime::now(),
                Source::Worker(self.id),
                EventPayload::TaskOrdered {
                    task_id: task_id,
                    worker_id: self.id,
                },
            ))
            .unwrap();

        self.tasks_events_sender
            .send(TaskEvent::new(self.id, task_id, TaskInput { input: 41 }))
            .unwrap();

        return 41;
    }
}

impl fmt::Display for Worker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} id {}", WORKER, self.id)
    }
}
