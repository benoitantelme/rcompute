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
    // task_timeout: Option<Duration>,
    // timeout_check_frequency: Duration,
    pub open_calculations: HashSet<u32>,
    pub results: HashMap<u32, Result<[[u32; MATRIX_SIZE]; MATRIX_SIZE], String>>,
}

impl Application {
    pub fn new(id: u32, config: AppConfig) -> Self {
        Self {
            id,
            config: config,
            // monitor_display: false,
            open_calculations: HashSet::new(),
            // task_timeout: None,
            // timeout_check_frequency: Duration::from_secs(1),
            results: HashMap::new(),
        }
    }

    // pub fn from_config(id: u32, config: AppConfig) -> Self {
    //     let mut application = Self::new(id, config.matrix_size);

    //     application.task_timeout =
    //         (config.timeout != 0).then(|| Duration::from_millis(config.timeout));

    //     // A zero frequency is invalid for a periodic poll and would cause a
    //     // busy loop, so use a small safe interval in that case.
    //     application.timeout_check_frequency = Duration::from_millis(config.check_frequency.max(1));
    //     application.monitor_display = config.monitor_display;
    //     application
    // }

    pub fn multiply(
        mut self,
        a: [[u32; MATRIX_SIZE]; MATRIX_SIZE],
        b: [[u32; MATRIX_SIZE]; MATRIX_SIZE],
    ) -> [[u32; MATRIX_SIZE]; MATRIX_SIZE] {
        let (monitor_tx, monitor_rx) = mpsc::channel::<MonitorEvent>();
        let monitor = Monitor::new(1, monitor_rx, self.config.monitor_display);

        std::thread::spawn(move || monitor.run());

        let matrix_size = self.config.matrix_size;
        let (task_tx, task_rx) = mpsc::channel::<TaskEvent>();
        let mut orchestrator =
            Orchestrator::from_config(1, monitor_tx.clone(), task_rx, self.config);
        println!("{}", orchestrator.to_string());

        for worker_id in 1..=(matrix_size * matrix_size) as u32 {
            let (worker, work_sender) =
                Worker::with_work_channel(worker_id, task_tx.clone(), monitor_tx.clone());
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
