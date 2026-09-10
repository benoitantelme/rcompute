use rcompute::components::event::MonitorEvent;
use rcompute::components::monitor::Monitor;
use rcompute::components::orchestrator::Orchestrator;
use rcompute::components::task::TaskEvent;
use rcompute::components::worker::Worker;
use rcompute::config::app_config::AppConfig;

use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (monitor_tx, monitor_rx) = mpsc::channel::<MonitorEvent>();
    let monitor = Monitor::new(1, monitor_rx);

    std::thread::spawn(move || monitor.run());

    let config = AppConfig::read_config();
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

    let (worker, work_sender) = Worker::with_work_channel(1, task_tx, monitor_tx.clone());
    orchestrator.register_worker_channel(1, work_sender);
    orchestrator.dispatch_multiply(1, 6, 7, 0).unwrap();

    println!("{}", worker.to_string());
    std::thread::spawn(move || worker.run());
    std::thread::spawn(move || orchestrator.run());
    std::thread::sleep(Duration::from_millis(50));
}
