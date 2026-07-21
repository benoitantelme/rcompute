#[cfg(test)]
mod worker_test {
    use rcompute::components::event::TaskEvent;
    use rcompute::components::worker::Worker;

    use std::sync::mpsc;
    use std::time::Duration;

    #[test]
    fn test_workers() {
        let (tx, rx) = mpsc::channel::<TaskEvent>();
        let worker = Worker::new(1, 1, tx);
        println!("{}", worker.to_string());
        worker.calculate();
        std::thread::sleep(Duration::from_millis(10));

        let event = rx.recv().unwrap();
        match event {
            TaskEvent::TaskFinished(result) => {
                assert_eq!(result.id, 1);
                assert_eq!(result.result, 42);
            }
            _ => panic!("We should not receive something else than a TaskFinished event"),
        }
    }
}
