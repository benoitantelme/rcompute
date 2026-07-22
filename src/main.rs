use rcompute::components::event::MonitorEvent;
use rcompute::components::event::TaskEvent;
use rcompute::components::monitor::Monitor;
use rcompute::components::orchestrator::Orchestrator;
use rcompute::components::worker::Worker;
use rcompute::config::app_config::AppConfig;

use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (monitor_tx, monitor_rx) = mpsc::channel::<MonitorEvent>();
    let monitor = Monitor::new(1, monitor_rx);

    std::thread::spawn(move || monitor.run());

    let config: AppConfig = AppConfig::new();
    let (task_tx, task_rx) = mpsc::channel::<TaskEvent>();
    let mut orchestrator = Orchestrator::new(
        1,
        monitor_tx.clone(),
        task_rx,
        config.workers_number,
        config.workers_threshold,
        config.timeout,
        config.check_frequency,
    );
    println!("{}", orchestrator.to_string());
    orchestrator.initialise();

    std::thread::spawn(move || orchestrator.run());

    let worker = Worker::new(1, 1, task_tx.clone(), monitor_tx.clone());
    println!("{}", worker.to_string());
    worker.calculate();
    std::thread::sleep(Duration::from_millis(50));
}
