use crate::components::event::EventPayload;
use crate::components::event::MonitorEvent;
use crate::components::event::Source;

use std::fmt;
use std::sync::{Arc, RwLock, mpsc};
use std::time::Duration;

const MONITOR: &str = "Monitor: ";

pub struct Monitor {
    pub id: u32,
    pub events: Arc<RwLock<Vec<MonitorEvent>>>,
    receiver: mpsc::Receiver<MonitorEvent>,
}

impl Monitor {
    pub fn new(id: u32, receiver: mpsc::Receiver<MonitorEvent>) -> Self {
        Self {
            id: id,
            receiver: receiver,
            events: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn run(self) {
        println!("Monitor {} starting", self.id);
        loop {
            while let Ok(event) = self.receiver.try_recv() {
                match &event.payload {
                    EventPayload::TaskAssigned { task_id } => {
                        println!("{}Task assigned {}", MONITOR, task_id);
                    }
                    EventPayload::TaskStarted { task_id } => {
                        println!("{}Task started {}", MONITOR, task_id);
                    }
                    EventPayload::TaskCompleted { task_id } => {
                        println!("{}Task completed {}", MONITOR, task_id);
                    }
                    EventPayload::TaskFailed { task_id, reason } => {
                        println!(
                            "{}Task failed with id {} because {}",
                            MONITOR, task_id, reason
                        )
                    }
                }

                let mut writable_events = self.events.write().unwrap();
                writable_events.push(event);
            }

            std::thread::sleep(Duration::from_millis(5));
        }
    }

    // async?
    pub fn history(&self) -> Vec<MonitorEvent> {
        println!("{} {} returning history", MONITOR, self.id);
        self.events.read().unwrap().clone()
    }

    pub async fn events_from_worker(&self, worker_id: u32) -> Vec<MonitorEvent> {
        println!(
            "{} {} returning history for worker {}",
            MONITOR, self.id, worker_id
        );
        let snapshot = self.events.read().unwrap().clone();

        snapshot
            .into_iter()
            .filter_map(|event| match event.source {
                Source::Worker(id) => {
                    if id == worker_id {
                        Some(event)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect()
    }
}

impl fmt::Display for Monitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} id {}", MONITOR, self.id)
    }
}
