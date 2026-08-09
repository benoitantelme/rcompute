use crate::components::event::EventPayload;
use crate::components::event::MonitorEvent;
use crate::components::event::Source;

use std::fmt;
use std::sync::{Arc, RwLock, mpsc};
use std::time::Duration;

const MONITOR: &str = "Monitor: ";

pub struct Monitor {
    pub id: u32,
    pub orchestrator_events: Arc<RwLock<Vec<MonitorEvent>>>,
    pub workers_events: Arc<RwLock<Vec<MonitorEvent>>>,
    receiver: mpsc::Receiver<MonitorEvent>,
}

impl Monitor {
    pub fn new(id: u32, receiver: mpsc::Receiver<MonitorEvent>) -> Self {
        Self {
            id: id,
            receiver: receiver,
            orchestrator_events: Arc::new(RwLock::new(Vec::new())),
            workers_events: Arc::new(RwLock::new(Vec::new())),
        }
    }

    fn get_history(&self, source: &Source) -> &Arc<RwLock<Vec<MonitorEvent>>> {
        println!("{} {} returning {} history", MONITOR, self.id, source);
        let events_history: &Arc<RwLock<Vec<MonitorEvent>>> = match source {
            Source::Orchestrator => &self.orchestrator_events,
            Source::Worker(_) => &self.workers_events,
        };

        events_history
    }

    pub fn run(self) {
        println!("Monitor {} starting", self.id);
        loop {
            while let Ok(event) = self.receiver.try_recv() {
                match &event.payload {
                    EventPayload::TaskAssigned { task_id, worker_id } => {
                        println!(
                            "{}Task from {} assigned {} to {}",
                            MONITOR, event.source, task_id, worker_id
                        );
                    }
                    EventPayload::TaskStarted { task_id, worker_id } => {
                        println!(
                            "{}Task from {} started {} by {}",
                            MONITOR, event.source, task_id, worker_id
                        );
                    }
                    EventPayload::TaskCompleted { task_id, worker_id } => {
                        println!(
                            "{}Task from {} completed {} by {}",
                            MONITOR, event.source, task_id, worker_id
                        );
                    }
                    EventPayload::TaskDuplicated { task_id, worker_id } => {
                        println!(
                            "{}Task from {} duplicated {} by {}",
                            MONITOR, event.source, task_id, worker_id
                        );
                    }
                    EventPayload::TaskOrdered { task_id, worker_id } => {
                        println!(
                            "{}Task from {} ordered {} by {}",
                            MONITOR, event.source, task_id, worker_id
                        );
                    }
                    EventPayload::TaskFailed {
                        task_id,
                        worker_id,
                        reason,
                    } => {
                        println!(
                            "{}Task from {} failed with id {} by {} because {}",
                            MONITOR, event.source, task_id, worker_id, reason
                        )
                    }
                }

                let events_history = self.get_history(&event.source);
                events_history.write().unwrap().push(event);
            }

            std::thread::sleep(Duration::from_millis(5));
        }
    }

    // async?
    pub fn history(&self, source: Source) -> Vec<MonitorEvent> {
        println!("{} {} returning {} history", MONITOR, self.id, source);
        let events_history = self.get_history(&source);
        events_history.read().unwrap().clone()
    }

    pub async fn events_from_worker(&self, worker_id: u32) -> Vec<MonitorEvent> {
        println!(
            "{} {} returning history for worker {}",
            MONITOR, self.id, worker_id
        );
        let snapshot = self.workers_events.read().unwrap().clone();

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
