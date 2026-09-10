#[cfg(test)]
mod worker_test {
    use rcompute::components::event::MonitorEvent;
    use rcompute::components::task::{Task, TaskEvent};
    use rcompute::components::worker::Worker;

    use std::sync::mpsc;

    #[test]
    fn test_workers() {
        let (task_tx, task_rx) = mpsc::channel::<TaskEvent>();
        let (monitor_tx, _monitor_rx) = mpsc::channel::<MonitorEvent>();
        let (worker, work_tx) = Worker::with_work_channel(1, task_tx, monitor_tx);
        println!("{}", worker.to_string());
        std::thread::spawn(move || worker.run());
        work_tx
            .send(TaskEvent::new(1, 1, Task::Multiply { a: 6, b: 7, k: 0 }))
            .unwrap();

        let event = task_rx.recv().unwrap();
        match event.task {
            Task::PartialResult { value, k } => {
                assert_eq!(event.task_id, 1);
                assert_eq!(value, 42);
                assert_eq!(k, 0);
            }
            _ => panic!("We should receive a partial result"),
        }
    }
}
