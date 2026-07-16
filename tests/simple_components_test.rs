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

        orchestrator.workers.push_back(worker1);
        orchestrator.workers.push_back(worker2);
        orchestrator.workers.push_back(worker3);
        orchestrator.workers.push_back(worker4);

        assert_eq!(orchestrator.workers.len(), 4);

        for n in 1..5 {
            let worker = orchestrator.workers.pop_front().unwrap();
            assert_eq!(worker.id, n);
            assert_eq!(worker.task, n);
            assert_eq!(worker.calculate(), 42);
        }

        assert_eq!(orchestrator.workers.len(), 0);

        orchestrator.workers.push_back(Worker::new(1, 1));
        let worker = orchestrator.workers.pop_front().unwrap();
        assert_eq!(worker.id, 1);
        assert_eq!(worker.task, 1);
        assert_eq!(worker.calculate(), 42);
        assert_eq!(orchestrator.workers.len(), 0);
    }
}
