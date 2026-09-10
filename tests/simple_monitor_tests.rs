use rcompute::components::event::{EventPayload, MonitorEvent, Source};
use rcompute::components::monitor::Monitor;
use rcompute::components::orchestrator::Orchestrator;
use rcompute::components::task::TaskEvent;
use rcompute::components::worker::Worker;
use std::sync::mpsc;
use std::time::Duration;

#[test]
fn monitor_records_orchestrator_and_worker_summa_events() {
    let (monitor_sender, monitor_receiver) = mpsc::channel::<MonitorEvent>();
    let (result_sender, result_receiver) = mpsc::channel::<TaskEvent>();
    let monitor = Monitor::new(1, monitor_receiver);
    let orchestrator_events = monitor.orchestrator_events.clone();
    let worker_events = monitor.workers_events.clone();
    std::thread::spawn(move || monitor.run());

    let mut orchestrator = Orchestrator::new(1, monitor_sender.clone(), result_receiver, 3);
    for worker_id in 1..=9 {
        let (worker, work_sender) =
            Worker::with_work_channel(worker_id, result_sender.clone(), monitor_sender.clone());
        orchestrator.register_worker_channel(worker_id, work_sender);
        std::thread::spawn(move || worker.run());
    }

    orchestrator
        .multiply_summa(
            [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
            [[9, 8, 7], [6, 5, 4], [3, 2, 1]],
        )
        .unwrap();

    std::thread::sleep(Duration::from_millis(20));
    let orchestrator_events = orchestrator_events.read().unwrap();
    assert_eq!(
        orchestrator_events
            .iter()
            .filter(|event| {
                event.source == Source::Orchestrator
                    && matches!(event.payload, EventPayload::TaskAssigned { .. })
            })
            .count(),
        27
    );
    assert_eq!(
        orchestrator_events
            .iter()
            .filter(|event| {
                event.source == Source::Orchestrator
                    && matches!(event.payload, EventPayload::TaskCompleted { .. })
            })
            .count(),
        27
    );

    let worker_events = worker_events.read().unwrap();
    assert_eq!(
        worker_events
            .iter()
            .filter(|event| matches!(event.payload, EventPayload::TaskStarted { .. }))
            .count(),
        27
    );
    assert_eq!(
        worker_events
            .iter()
            .filter(|event| matches!(event.payload, EventPayload::TaskCompleted { .. }))
            .count(),
        27
    );
}
