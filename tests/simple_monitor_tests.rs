#[cfg(test)]
mod simple_monitor_test {
    use rcompute::components::event::{EventPayload, MonitorEvent, Source, TaskEvent};
    use rcompute::components::monitor::Monitor;
    use rcompute::components::orchestrator::Orchestrator;
    use rcompute::components::worker::Worker;

    use std::sync::mpsc;
    use std::{thread, time};

    #[test]
    fn initialisation() {
        let (monitor_tx, monitor_rx) = mpsc::channel::<MonitorEvent>();
        let (task_tx, task_rx) = mpsc::channel::<TaskEvent>();

        let monitor = Monitor::new(1, monitor_rx);
        // Clone only the shared history
        let history_clone = monitor.events.clone();
        std::thread::spawn(move || monitor.run());

        let mut orchestrator = Orchestrator::new(1, monitor_tx.clone(), task_rx, 5, 3, 30, 30);

        orchestrator.initialise();

        std::thread::spawn(move || orchestrator.run());

        let worker = Worker::new(1, 1, task_tx.clone(), monitor_tx.clone());
        println!("{}", worker.to_string());
        worker.calculate();

        thread::sleep(time::Duration::from_millis(100));
        let mut history = history_clone.read().unwrap().clone();
        assert_eq!(history.len(), 7);

        // orchestrator timeouts message at the end for the 5th timed out workers
        for n in 1..6 {
            let failed = history.pop().unwrap();
            assert_eq!(failed.id, 1);
            assert_eq!(failed.source, Source::Orchestrator);
            assert_eq!(
                failed.payload,
                EventPayload::TaskFailed {
                    task_id: 6 - n,
                    reason: "Timeout".to_string()
                }
            );
        }

        // then calculated messages in reverse order
        let second = history.pop().unwrap();
        assert_eq!(second.id, 1);
        assert_eq!(second.source, Source::Orchestrator);
        assert_eq!(second.payload, EventPayload::TaskCompleted { task_id: 1 });

        let first = history.pop().unwrap();
        assert_eq!(first.id, 1);
        assert_eq!(first.source, Source::Worker(1));
        assert_eq!(first.payload, EventPayload::TaskCompleted { task_id: 1 });
    }
}
