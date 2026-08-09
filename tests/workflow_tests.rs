#[cfg(test)]
mod workflow_test {
    use rcompute::components::event::{EventPayload, MonitorEvent, Source};
    use rcompute::components::monitor::Monitor;
    use rcompute::components::orchestrator::Orchestrator;
    use rcompute::components::task::TaskEvent;
    use rcompute::components::worker::Worker;

    use std::sync::mpsc;
    use std::{thread, time};

    #[test]
    fn clean_workflow_test() {
        let (monitor_tx, monitor_rx) = mpsc::channel::<MonitorEvent>();
        let (task_tx, task_rx) = mpsc::channel::<TaskEvent>();

        let monitor = Monitor::new(1, monitor_rx);
        // Clone only the shared history
        let orchestrator_history_clone = monitor.orchestrator_events.clone();
        let workers_history_clone = monitor.workers_events.clone();
        std::thread::spawn(move || monitor.run());

        let mut orchestrator =
            Orchestrator::new(1, monitor_tx.clone(), task_rx, 5, 3, 99999999999, 90000000);

        orchestrator.initialise();

        std::thread::spawn(move || orchestrator.run());

        let worker = Worker::new(1, task_tx.clone(), monitor_tx.clone());
        worker.calculate(1);

        let worker2 = Worker::new(2, task_tx.clone(), monitor_tx.clone());
        worker2.calculate(2);

        let worker3 = Worker::new(3, task_tx.clone(), monitor_tx.clone());
        worker3.send_task(3, 4);
        worker3.send_task(4, 5);
        worker3.send_task(5, 6);

        thread::sleep(time::Duration::from_millis(100));

        worker.timeout(1);
        worker2.timeout(2);
        worker3.timeout(3);

        thread::sleep(time::Duration::from_millis(100));
        let mut workers_history = workers_history_clone.read().unwrap().clone();
        assert_eq!(workers_history.len(), 8);

        // reverse order
        let eight = workers_history.pop().unwrap();
        assert_eq!(eight.id, 3);
        assert_eq!(eight.source, Source::Worker(3));
        assert_eq!(
            eight.payload,
            EventPayload::TaskFailed {
                task_id: 3,
                worker_id: 3,
                reason: "Timeout".to_string()
            }
        );

        let seven = workers_history.pop().unwrap();
        assert_eq!(seven.id, 2);
        assert_eq!(seven.source, Source::Worker(2));
        assert_eq!(
            seven.payload,
            EventPayload::TaskFailed {
                task_id: 2,
                worker_id: 2,
                reason: "Timeout".to_string()
            }
        );

        let six = workers_history.pop().unwrap();
        assert_eq!(six.id, 1);
        assert_eq!(six.source, Source::Worker(1));
        assert_eq!(
            six.payload,
            EventPayload::TaskFailed {
                task_id: 1,
                worker_id: 1,
                reason: "Timeout".to_string()
            }
        );

        let five = workers_history.pop().unwrap();
        assert_eq!(five.id, 3);
        assert_eq!(five.source, Source::Worker(3));
        assert_eq!(
            five.payload,
            EventPayload::TaskOrdered {
                task_id: 5,
                worker_id: 3
            }
        );

        let four = workers_history.pop().unwrap();
        assert_eq!(four.id, 3);
        assert_eq!(four.source, Source::Worker(3));
        assert_eq!(
            four.payload,
            EventPayload::TaskOrdered {
                task_id: 4,
                worker_id: 3
            }
        );

        let three = workers_history.pop().unwrap();
        assert_eq!(three.id, 3);
        assert_eq!(three.source, Source::Worker(3));
        assert_eq!(
            three.payload,
            EventPayload::TaskOrdered {
                task_id: 3,
                worker_id: 3
            }
        );

        let two = workers_history.pop().unwrap();
        assert_eq!(two.id, 2);
        assert_eq!(two.source, Source::Worker(2));
        assert_eq!(
            two.payload,
            EventPayload::TaskCompleted {
                task_id: 2,
                worker_id: 2
            }
        );

        let one = workers_history.pop().unwrap();
        assert_eq!(one.id, 1);
        assert_eq!(one.source, Source::Worker(1));
        assert_eq!(
            one.payload,
            EventPayload::TaskCompleted {
                task_id: 1,
                worker_id: 1
            }
        );

        // orchestrator side
        let mut orchestrator_history = orchestrator_history_clone.read().unwrap().clone();
        assert_eq!(orchestrator_history.len(), 8);

        // reverse order
        let eight = orchestrator_history.pop().unwrap();
        assert_eq!(eight.id, 1);
        assert_eq!(eight.source, Source::Orchestrator);
        assert_eq!(
            eight.payload,
            EventPayload::TaskFailed {
                task_id: 3,
                worker_id: 3,
                reason: "Timeout".to_string()
            }
        );

        let seven = orchestrator_history.pop().unwrap();
        assert_eq!(seven.id, 1);
        assert_eq!(seven.source, Source::Orchestrator);
        assert_eq!(
            seven.payload,
            EventPayload::TaskFailed {
                task_id: 2,
                worker_id: 2,
                reason: "Timeout".to_string()
            }
        );

        let six = orchestrator_history.pop().unwrap();
        assert_eq!(six.id, 1);
        assert_eq!(six.source, Source::Orchestrator);
        assert_eq!(
            six.payload,
            EventPayload::TaskFailed {
                task_id: 1,
                worker_id: 1,
                reason: "Timeout".to_string()
            }
        );

        let five = orchestrator_history.pop().unwrap();
        assert_eq!(five.id, 1);
        assert_eq!(five.source, Source::Orchestrator);
        assert_eq!(
            five.payload,
            EventPayload::TaskStarted {
                task_id: 5,
                worker_id: 3
            }
        );

        let four = orchestrator_history.pop().unwrap();
        assert_eq!(four.id, 1);
        assert_eq!(four.source, Source::Orchestrator);
        assert_eq!(
            four.payload,
            EventPayload::TaskStarted {
                task_id: 4,
                worker_id: 3
            }
        );

        let three = orchestrator_history.pop().unwrap();
        assert_eq!(three.id, 1);
        assert_eq!(three.source, Source::Orchestrator);
        assert_eq!(
            three.payload,
            EventPayload::TaskStarted {
                task_id: 3,
                worker_id: 3
            }
        );

        let two = orchestrator_history.pop().unwrap();
        assert_eq!(two.id, 1);
        assert_eq!(two.source, Source::Orchestrator);
        assert_eq!(
            two.payload,
            EventPayload::TaskCompleted {
                task_id: 2,
                worker_id: 2
            }
        );

        let one = orchestrator_history.pop().unwrap();
        assert_eq!(one.id, 1);
        assert_eq!(one.source, Source::Orchestrator);
        assert_eq!(
            one.payload,
            EventPayload::TaskCompleted {
                task_id: 1,
                worker_id: 1
            }
        );
    }
}
