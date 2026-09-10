use rcompute::components::event::{EventPayload, MonitorEvent, Source};
use rcompute::components::monitor::Monitor;
use rcompute::components::orchestrator::Orchestrator;
use rcompute::components::task::TaskEvent;
use rcompute::components::worker::Worker;
use std::sync::mpsc;
use std::time::Duration;

#[test]
fn monitor_records_the_multiply_lifecycle() {
    let (monitor_sender, monitor_receiver) = mpsc::channel::<MonitorEvent>();
    let (result_sender, result_receiver) = mpsc::channel::<TaskEvent>();
    let monitor = Monitor::new(1, monitor_receiver);
    let orchestrator_events = monitor.orchestrator_events.clone();
    let worker_events = monitor.workers_events.clone();
    std::thread::spawn(move || monitor.run());

    let (worker, work_sender) = Worker::with_work_channel(1, result_sender, monitor_sender.clone());
    let mut orchestrator = Orchestrator::new(1, monitor_sender, result_receiver, 1, 1, 1_000, 100);
    orchestrator.initialise();
    orchestrator.register_worker_channel(1, work_sender);
    std::thread::spawn(move || worker.run());
    orchestrator.dispatch_multiply(1, 6, 7, 0).unwrap();
    std::thread::sleep(Duration::from_millis(10));
    orchestrator.process_incoming_tasks();
    std::thread::sleep(Duration::from_millis(10));

    let worker_events = worker_events.read().unwrap();
    assert!(worker_events.iter().any(|event| {
        event.source == Source::Worker(1)
            && event.payload
                == EventPayload::TaskStarted {
                    task_id: 1,
                    worker_id: 1,
                }
    }));
    assert!(worker_events.iter().any(|event| {
        event.source == Source::Worker(1)
            && event.payload
                == EventPayload::TaskCompleted {
                    task_id: 1,
                    worker_id: 1,
                }
    }));
    let orchestrator_events = orchestrator_events.read().unwrap();
    assert!(orchestrator_events.iter().any(|event| {
        event.source == Source::Orchestrator
            && event.payload
                == EventPayload::TaskAssigned {
                    task_id: 1,
                    worker_id: 1,
                }
    }));
    assert!(orchestrator_events.iter().any(|event| {
        event.source == Source::Orchestrator
            && event.payload
                == EventPayload::TaskCompleted {
                    task_id: 1,
                    worker_id: 1,
                }
    }));
}
