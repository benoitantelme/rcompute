#[cfg(test)]
mod timer_test {
    use rcompute::components::event::{EventPayload, MonitorEvent, Source};
    use rcompute::components::monitor::Monitor;
    use rcompute::components::orchestrator::Orchestrator;
    use rcompute::components::task::TaskEvent;
    use rcompute::components::timer::Deadline;
    use rcompute::components::worker::Worker;

    use std::sync::mpsc;
    use std::{thread, time};

    #[test]
    fn check_expiration() {
        let deadline = Deadline::new(1, 1, 100);
        assert_eq!(deadline.is_expired(), false);
        thread::sleep(time::Duration::from_millis(10));
        assert_eq!(deadline.is_expired(), false);
        thread::sleep(time::Duration::from_millis(100));
        assert_eq!(deadline.is_expired(), true);
    }

    #[test]
    fn check_values() {
        let deadline = Deadline::new(1, 1, 100);
        assert_eq!(deadline.task_id, 1);
        assert!(deadline.when < time::SystemTime::now() + time::Duration::from_millis(100));
    }

    #[test]
    fn check_ordering() {
        let (monitor_tx, _monitor_rx) = mpsc::channel::<MonitorEvent>();
        let (_tx, rx) = mpsc::channel::<TaskEvent>();
        let mut orchestrator = Orchestrator::new(1, monitor_tx.clone(), rx, 5, 3, 30, 30);
        orchestrator.initialise();

        for n in 1..5 {
            let deadline = Deadline::new(1, n, n as u64 * 100);
            orchestrator.deadlines.push(deadline);
        }

        for n in 1..5 {
            let deadline = orchestrator.deadlines.pop().unwrap();
            assert_eq!(deadline.worker_id, n);
        }
    }

    #[test]
    fn orchestrator_timeouts() {
        let (monitor_tx, monitor_rx) = mpsc::channel::<MonitorEvent>();
        let (task_tx, task_rx) = mpsc::channel::<TaskEvent>();

        let monitor = Monitor::new(1, monitor_rx);
        let orchestrator_history_clone = monitor.orchestrator_events.clone();
        let workers_history_clone = monitor.workers_events.clone();
        std::thread::spawn(move || monitor.run());

        // no initial capacity and long timeout to avoid orchestrator deadline timeout to kick in before the worker timeout
        let mut orchestrator = Orchestrator::new(1, monitor_tx.clone(), task_rx, 0, 0, 1000, 10);
        orchestrator.initialise();
        std::thread::spawn(move || orchestrator.run());

        let worker = Worker::new(1, task_tx.clone(), monitor_tx.clone());
        println!("{}", worker.to_string());
        worker.timeout(1);

        thread::sleep(time::Duration::from_millis(100));
        let mut orchestrator_history = orchestrator_history_clone.read().unwrap().clone();
        let mut workers_history = workers_history_clone.read().unwrap().clone();
        assert_eq!(orchestrator_history.len(), 1);
        assert_eq!(workers_history.len(), 1);

        // orchestrator timeout message  
        let first = orchestrator_history.pop().unwrap();
        assert_eq!(first.id, 1);
        assert_eq!(first.source, Source::Orchestrator);
        assert_eq!(
            first.payload,
            EventPayload::TaskFailed {
                task_id: 1,
                worker_id: 1,
                reason: "Timeout".to_string()
            }
        );

        // worker timeouts message at the beginning of the history
        let second = workers_history.pop().unwrap();
        assert_eq!(second.id, 1);
        assert_eq!(second.source, Source::Worker(1));
        assert_eq!(
            second.payload,
            EventPayload::TaskFailed {
                task_id: 1,
                worker_id: 1,
                reason: "Timeout".to_string()
            }
        );
    }
}
