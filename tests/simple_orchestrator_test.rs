#[cfg(test)]
mod simple_orchestrator_test {
    use rcompute::components::event::Event;
    use rcompute::components::orchestrator::Orchestrator;

    use std::sync::mpsc;

    #[test]
    fn init() {
        let (tx, rx) = mpsc::channel::<Event>();
        let mut orchestrator = Orchestrator::new(1, rx, 5, 3, 30, 30);

        orchestrator.initialise();
        assert_eq!(orchestrator.get_worker_queue_size(), 5);
    }

    #[test]
    fn threshold_test() {
        let (tx, rx) = mpsc::channel::<Event>();
        let mut orchestrator = Orchestrator::new(1, rx, 5, 3, 30, 30);

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

    #[test]
    #[should_panic]
    fn availability_test() {
        let (tx, rx) = mpsc::channel::<Event>();
        let mut orchestrator = Orchestrator::new(1, rx, 5, 3, 30, 30);

        orchestrator.initialise();

        for _n in 1..6 {
            orchestrator.pull_worker();
        }

        orchestrator.pull_worker();
    }
}
