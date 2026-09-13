use crate::components::event::MonitorEvent;
use crate::components::monitor::Monitor;
use crate::components::orchestrator::Orchestrator;
use crate::components::task::TaskEvent;
use crate::components::worker::Worker;
use crate::config::app_config::AppConfig;

use std::collections::{HashMap, HashSet};
use std::sync::mpsc;

const MATRIX_SIZE: usize = 3;

pub struct Application {
    pub id: u32,
    // pub matrix_size: usize,
    config: AppConfig,
    monitor: Monitor,
    monitor_tx: mpsc::Sender<MonitorEvent>,
    pub open_calculations: HashSet<u32>,
    pub results: HashMap<u32, Result<[[u32; MATRIX_SIZE]; MATRIX_SIZE], String>>,
}

impl Application {
    pub fn new(id: u32, config: AppConfig) -> Self {
        let (monitor_tx, monitor_rx) = mpsc::channel::<MonitorEvent>();
        let monitor = Monitor::new(1, monitor_rx, config.monitor_display);

        Self {
            id,
            config: config,
            monitor,
            monitor_tx,
            open_calculations: HashSet::new(),
            results: HashMap::new(),
        }
    }

    pub fn init(self) {
        std::thread::spawn(move || self.monitor.run());
    }

    pub fn multiply(
        mut self,
        a: [[u32; MATRIX_SIZE]; MATRIX_SIZE],
        b: [[u32; MATRIX_SIZE]; MATRIX_SIZE],
    ) -> [[u32; MATRIX_SIZE]; MATRIX_SIZE] {
        let matrix_size = self.config.matrix_size;
        let (task_tx, task_rx) = mpsc::channel::<TaskEvent>();
        let mut orchestrator =
            Orchestrator::from_config(1, self.monitor_tx.clone(), task_rx, self.config);
        println!("{}", orchestrator.to_string());

        for worker_id in 1..=(matrix_size * matrix_size) as u32 {
            let (worker, work_sender) =
                Worker::with_work_channel(worker_id, task_tx.clone(), self.monitor_tx.clone());
            orchestrator.register_worker_channel(worker_id, work_sender);
            println!("{}", worker);
            std::thread::spawn(move || worker.run());
        }

        let result = orchestrator.multiply_summa(a, b).unwrap();
        self.results.insert(1, Ok(result));
        self.open_calculations.remove(&1);

        return result;
    }
}
