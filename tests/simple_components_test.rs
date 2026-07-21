#[cfg(test)]
mod simple_components_test {
    use rcompute::components::event::TaskEvent;
    use rcompute::components::orchestrator::Orchestrator;
    use rcompute::components::worker::Worker;

    use std::sync::mpsc;

    #[test]
    fn instantiation() {
        let (tx, rx) = mpsc::channel::<TaskEvent>();
        let orchestrator = Orchestrator::new(1, rx, 10, 3, 30, 30);
        assert_eq!(orchestrator.id, 1);
        assert_eq!(orchestrator.threshold, 3);
        assert_eq!(orchestrator.timeout, 30);
        assert_eq!(orchestrator.check_frequency, 30);

        let worker = Worker::new(1, 1, tx);
        assert_eq!(worker.id, 1);
        assert_eq!(worker.task, 1);
    }

    #[test]
    fn queuing() {
        let (_tx, rx) = mpsc::channel::<TaskEvent>();
        let mut orchestrator = Orchestrator::new(1, rx, 10, 3, 30, 30);

        for n in 1..5 {
            orchestrator.push_worker(n);
        }

        assert_eq!(orchestrator.get_worker_queue_size(), 4);

        for n in 1..5 {
            let worker_id = orchestrator.pull_worker();
            assert_eq!(worker_id, n);
        }

        assert_eq!(orchestrator.get_worker_queue_size(), 0);

        orchestrator.push_worker(1);
        let worker_id = orchestrator.pull_worker();
        assert_eq!(worker_id, 1);
        assert_eq!(orchestrator.get_worker_queue_size(), 0);
    }
}
