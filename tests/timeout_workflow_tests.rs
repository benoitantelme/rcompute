use rcompute::components::event::MonitorEvent;
use rcompute::components::orchestrator::Orchestrator;
use rcompute::components::task::TaskEvent;
use rcompute::components::worker::Worker;
use rcompute::config::app_config::AppConfig;
use std::sync::mpsc;

#[test]
fn overdue_multiplications_are_recorded_as_failed() {
    let (monitor_sender, _monitor_receiver) = mpsc::channel::<MonitorEvent>();
    let (result_sender, result_receiver) = mpsc::channel::<TaskEvent>();
    let mut config = AppConfig::new();
    config.set_config(3, 5, 1, false);
    let mut orchestrator =
        Orchestrator::from_config(1, monitor_sender.clone(), result_receiver, config);

    // Keep worker 1's work receiver alive without running it. Its first
    // multiplication therefore has no result and must be timed out locally.
    let (stalled_sender, _stalled_receiver) = mpsc::channel::<TaskEvent>();
    orchestrator.register_worker_channel(1, stalled_sender);

    for worker_id in 2..=9 {
        let (worker, work_sender) =
            Worker::with_work_channel(worker_id, result_sender.clone(), monitor_sender.clone());
        orchestrator.register_worker_channel(worker_id, work_sender);
        std::thread::spawn(move || worker.run());
    }

    assert!(
        orchestrator
            .multiply_summa([[1; 3]; 3], [[1; 3]; 3])
            .is_err()
    );
    assert!(orchestrator.failed_tasks.contains(&1));
    assert!(!orchestrator.open_tasks.contains_key(&1));
}
