use rcompute::components::event::MonitorEvent;
use rcompute::components::orchestrator::Orchestrator;
use rcompute::components::task::TaskEvent;
use rcompute::components::worker::Worker;
use std::sync::mpsc;
use std::time::Duration;

#[test]
fn multiply_workflows_through_orchestrator_and_worker() {
    let (monitor_sender, _monitor_receiver) = mpsc::channel::<MonitorEvent>();
    let (result_sender, result_receiver) = mpsc::channel::<TaskEvent>();
    let (worker, work_sender) = Worker::with_work_channel(1, result_sender, monitor_sender.clone());
    let mut orchestrator = Orchestrator::new(1, monitor_sender, result_receiver, 1, 1, 1_000, 100);
    orchestrator.initialise();
    orchestrator.register_worker_channel(1, work_sender);
    std::thread::spawn(move || worker.run());

    assert_eq!(orchestrator.dispatch_multiply(1, 3, 14, 9), Ok(1));
    std::thread::sleep(Duration::from_millis(10));
    orchestrator.process_incoming_tasks();

    assert_eq!(orchestrator.partial_results.get(&9), Some(&42));
    assert!(orchestrator.closed_tasks.contains(&1));
}
