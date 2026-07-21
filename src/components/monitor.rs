use crate::components::event::MonitorEvent;
use crate::components::event::Source;

use std::sync::RwLock;
use std::sync::mpsc;

pub struct Monitor {
    events: RwLock<Vec<MonitorEvent>>,
    event_rx: mpsc::Receiver<MonitorEvent>,
}

impl Monitor {
    pub async fn history(&self) -> Vec<MonitorEvent> {
        self.events.read().unwrap().clone()
    }

    pub async fn events_from_worker(&self, worker_id: u32) -> Vec<MonitorEvent> {
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
