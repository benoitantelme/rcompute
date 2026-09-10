use rcompute::components::event::MonitorEvent;
use rcompute::components::orchestrator::Orchestrator;
use rcompute::components::task::TaskEvent;
use std::sync::mpsc;

#[test]
fn orchestrator_setup() {
    let (monitor_sender, _monitor_receiver) = mpsc::channel::<MonitorEvent>();
    let (_result_sender, result_receiver) = mpsc::channel::<TaskEvent>();
    let mut orchestrator = Orchestrator::new(1, monitor_sender, result_receiver);
    let (work_sender, _work_receiver) = mpsc::channel::<TaskEvent>();

    assert!(orchestrator.workers.is_empty());
    orchestrator.register_worker_channel(4, work_sender);
    assert_eq!(orchestrator.workers.len(), 1);
    assert!(orchestrator.workers.contains(&4));
    assert!(orchestrator.busy_workers.is_empty());
}
