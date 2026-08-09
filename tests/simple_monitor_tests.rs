#[cfg(test)]
mod simple_monitor_test {
    use rcompute::components::event::{EventPayload, MonitorEvent, Source};
    use rcompute::components::monitor::Monitor;
    use rcompute::components::orchestrator::Orchestrator;
    use rcompute::components::task::TaskEvent;
    use rcompute::components::worker::Worker;

    use std::sync::mpsc;
    use std::{thread, time};

    #[test]
    fn initialisation_test() {
        let (monitor_tx, monitor_rx) = mpsc::channel::<MonitorEvent>();
        let (task_tx, task_rx) = mpsc::channel::<TaskEvent>();

        let monitor = Monitor::new(1, monitor_rx);
        // Clone only the shared history
        let orchestrator_history_clone = monitor.orchestrator_events.clone();
        let workers_history_clone = monitor.workers_events.clone();
        std::thread::spawn(move || monitor.run());

        let mut orchestrator = Orchestrator::new(1, monitor_tx.clone(), task_rx, 5, 3, 30, 30);

        orchestrator.initialise();

        std::thread::spawn(move || orchestrator.run());

        let worker = Worker::new(1, task_tx.clone(), monitor_tx.clone());
        println!("{}", worker.to_string());
        worker.calculate(1);

        thread::sleep(time::Duration::from_millis(100));
        let mut history = orchestrator_history_clone.read().unwrap().clone();
        assert_eq!(history.len(), 6);

        // orchestrator timeouts message at the end for the 5th timed out workers
        for n in 1..6 {
            let failed = history.pop().unwrap();
            assert_eq!(failed.id, 1);
            assert_eq!(failed.source, Source::Orchestrator);
            assert_eq!(
                failed.payload,
                EventPayload::TaskFailed {
                    task_id: 42,
                    worker_id: 6 - n,
                    reason: "Timeout".to_string()
                }
            );
        }

        // then calculated messages in reverse order
        let first = history.pop().unwrap();
        assert_eq!(first.id, 1);
        assert_eq!(first.source, Source::Orchestrator);
        assert_eq!(
            first.payload,
            EventPayload::TaskCompleted {
                task_id: 1,
                worker_id: 1
            }
        );

        let mut workers_history = workers_history_clone.read().unwrap().clone();
        assert_eq!(workers_history.len(), 1);

        let first = workers_history.pop().unwrap();
        assert_eq!(first.id, 1);
        assert_eq!(first.source, Source::Worker(1));
        assert_eq!(
            first.payload,
            EventPayload::TaskCompleted {
                task_id: 1,
                worker_id: 1
            }
        );
    }

    #[test]
    fn duplicate_task_test() {
        let (monitor_tx, monitor_rx) = mpsc::channel::<MonitorEvent>();
        let (task_tx, task_rx) = mpsc::channel::<TaskEvent>();

        let monitor = Monitor::new(1, monitor_rx);
        // Clone only the shared history
        let orchestrator_history_clone = monitor.orchestrator_events.clone();
        let workers_history_clone = monitor.workers_events.clone();
        std::thread::spawn(move || monitor.run());

        let mut orchestrator = Orchestrator::new(1, monitor_tx.clone(), task_rx, 0, 3, 30, 30);

        orchestrator.initialise();

        std::thread::spawn(move || orchestrator.run());

        let worker = Worker::new(1, task_tx.clone(), monitor_tx.clone());
        println!("{}", worker.to_string());
        worker.send_task(1, 41);
        worker.send_task(1, 41);

        thread::sleep(time::Duration::from_millis(100));
        let mut history = orchestrator_history_clone.read().unwrap().clone();
        assert_eq!(history.len(), 2);

        // Last message is the orchestrator's response to the duplicate task
        let last = history.pop().unwrap();
        assert_eq!(last.id, 1);
        assert_eq!(last.source, Source::Orchestrator);
        assert_eq!(
            last.payload,
            EventPayload::TaskDuplicated {
                task_id: 1,
                worker_id: 1
            }
        );

        let previous_to_last = history.pop().unwrap();
        assert_eq!(previous_to_last.id, 1);
        assert_eq!(previous_to_last.source, Source::Orchestrator);
        assert_eq!(
            previous_to_last.payload,
            EventPayload::TaskStarted {
                task_id: 1,
                worker_id: 1
            }
        );

        let mut workers_history = workers_history_clone.read().unwrap().clone();
        // orchestrator timeouts message at the end for the 5th timed out workers
        for _n in 1..3 {
            let failed = workers_history.pop().unwrap();
            assert_eq!(failed.id, 1);
            assert_eq!(failed.source, Source::Worker(1));
            assert_eq!(
                failed.payload,
                EventPayload::TaskOrdered {
                    task_id: 1,
                    worker_id: 1,
                }
            );
        }
    }
}
