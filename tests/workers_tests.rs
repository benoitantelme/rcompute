#[cfg(test)]
mod worker_test {
    use rcompute::components::orchestrator::Orchestrator;

    #[test]
    fn check_workers_busy() {
        let mut orchestrator = Orchestrator::new(1, 5, 3, 30, 30);

        assert_eq!(orchestrator.busy_workers.len(), 0);
        orchestrator.initialise();
        assert_eq!(orchestrator.busy_workers.len(), 5);

        for _n in 1..3 {
            orchestrator.pull_worker();
        }
        assert_eq!(orchestrator.busy_workers.len(), 3);
    }
}
