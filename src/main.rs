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
    let matrix_size = config.matrix_size;
    let (task_tx, task_rx) = mpsc::channel::<TaskEvent>();
    let mut orchestrator = Orchestrator::from_config(1, monitor_tx.clone(), task_rx, config);
    println!("{}", orchestrator.to_string());

    for worker_id in 1..=(matrix_size * matrix_size) as u32 {
        let (worker, work_sender) =
            Worker::with_work_channel(worker_id, task_tx.clone(), monitor_tx.clone());
        orchestrator.register_worker_channel(worker_id, work_sender);
        println!("{}", worker);
        std::thread::spawn(move || worker.run());
    }

    let a = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let b = [[9, 8, 7], [6, 5, 4], [3, 2, 1]];
    let c = orchestrator.multiply_summa(a, b).unwrap();
    println!("SUMMA result: {c:?}");
    std::thread::sleep(Duration::from_millis(50));
}
