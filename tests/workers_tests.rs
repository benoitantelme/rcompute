#[cfg(test)]
mod worker_test {
    use rcompute::components::event::MonitorEvent;
    use rcompute::components::task::{Task, TaskEvent};
    use rcompute::components::worker::Worker;

    use std::sync::mpsc;
    use std::time::Duration;

    #[test]
    fn test_workers() {
        let (task_tx, task_rx) = mpsc::channel::<TaskEvent>();
        let (monitor_tx, _monitor_rx) = mpsc::channel::<MonitorEvent>();
        let worker = Worker::new(1, 1, task_tx.clone(), monitor_tx.clone());
        println!("{}", worker.to_string());
        worker.calculate();
        std::thread::sleep(Duration::from_millis(10));

        let event = task_rx.recv().unwrap();
        match event.task {
            Task::TaskResult { result } => {
                assert_eq!(event.task_id, 1);
                assert_eq!(result, 42);
            }
            _ => panic!("We should not receive something else than a TaskFinished event"),
        }
    }
}
