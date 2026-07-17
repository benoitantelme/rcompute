#[cfg(test)]
mod components_test {
    use rcompute::components::orchestrator::Orchestrator;
    use rcompute::components::worker::Worker;

    #[test]
    fn instantiation() {
        let orchestrator = Orchestrator::new(1, 10);
        assert_eq!(orchestrator.id, 1);

        let worker = Worker::new(1, 1);
        assert_eq!(worker.id, 1);
        assert_eq!(worker.task, 1);
    }

    #[test]
    fn queuing() {
        let mut orchestrator = Orchestrator::new(1, 10);

        let worker1 = Worker::new(1, 1);
        let worker2 = Worker::new(2, 2);
        let worker3 = Worker::new(3, 3);
        let worker4 = Worker::new(4, 4);

        orchestrator.push_worker(worker1);
        orchestrator.push_worker(worker2);
        orchestrator.push_worker(worker3);
        orchestrator.push_worker(worker4);

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
