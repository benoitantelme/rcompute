use rcompute::components::event::MonitorEvent;
use rcompute::components::orchestrator::Orchestrator;
use rcompute::components::task::TaskEvent;
use rcompute::components::worker::Worker;
use std::sync::mpsc;

#[test]
fn components_can_be_instantiated() {
    let (monitor_sender, _monitor_receiver) = mpsc::channel::<MonitorEvent>();
    let (result_sender, result_receiver) = mpsc::channel::<TaskEvent>();
    let orchestrator = Orchestrator::new(1, monitor_sender.clone(), result_receiver);
    let (worker, _work_sender) = Worker::with_work_channel(1, result_sender, monitor_sender);

    assert_eq!(orchestrator.id, 1);
    assert!(orchestrator.workers.is_empty());
    assert_eq!(worker.id, 1);
}
