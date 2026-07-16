#[cfg(test)]
mod components_test {
    use rcompute::components::orchestrator::Orchestrator;
    use rcompute::components::worker::Worker;

    #[test]
    fn instantiation() {
        let orchestrator = Orchestrator::new(1);
        assert_eq!(orchestrator.id, 1);

        let worker = Worker::new(1, 1, orchestrator);
        assert_eq!(worker.id, 1);
        assert_eq!(worker.task, 1);
    }
}
