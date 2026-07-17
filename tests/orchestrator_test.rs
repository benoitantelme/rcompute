#[cfg(test)]
mod orchestrator_test {
    use rcompute::components::orchestrator::Orchestrator;

    #[test]
    fn init() {
        let mut orchestrator = Orchestrator::new(1, 5, 3);

        orchestrator.initialise();
        assert_eq!(orchestrator.get_worker_queue_size(), 5);
    }

    #[test]
    fn threshold_test() {
        let mut orchestrator = Orchestrator::new(1, 5, 3);

        assert_eq!(orchestrator.low_capacity, true);
        orchestrator.initialise();
        assert_eq!(orchestrator.get_worker_queue_size(), 5);
        assert_eq!(orchestrator.low_capacity, false);

        for _n in 1..3 {
            orchestrator.pull_worker();
        }
        assert_eq!(orchestrator.low_capacity, false);
         orchestrator.pull_worker();
        assert_eq!(orchestrator.low_capacity, true);
    }
}
