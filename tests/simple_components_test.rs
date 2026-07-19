#[cfg(test)]
mod simple_components_test {
    use rcompute::components::orchestrator::Orchestrator;
    use rcompute::components::worker::Worker;

    #[test]
    fn instantiation() {
        let orchestrator = Orchestrator::new(1, 10, 3, 30, 30);
        assert_eq!(orchestrator.id, 1);
        assert_eq!(orchestrator.threshold, 3);
        assert_eq!(orchestrator.timeout, 30);
        assert_eq!(orchestrator.check_frequency, 30);

        let worker = Worker::new(1, 1);
        assert_eq!(worker.id, 1);
        assert_eq!(worker.task, 1);
    }

    #[test]
    fn queuing() {
        let mut orchestrator = Orchestrator::new(1, 10, 3, 30, 30);

        for n in 1..5 {
            let worker = Worker::new(n, n);
            orchestrator.push_worker(worker);
        }

        assert_eq!(orchestrator.get_worker_queue_size(), 4);

        for n in 1..5 {
            let worker = orchestrator.pull_worker();
            assert_eq!(worker.id, n);
            assert_eq!(worker.task, n);
            assert_eq!(worker.calculate(), 42);
        }

        assert_eq!(orchestrator.get_worker_queue_size(), 0);

        orchestrator.push_worker(Worker::new(1, 1));
        let worker = orchestrator.pull_worker();
        assert_eq!(worker.id, 1);
        assert_eq!(worker.task, 1);
        assert_eq!(worker.calculate(), 42);
        assert_eq!(orchestrator.get_worker_queue_size(), 0);
    }
}
