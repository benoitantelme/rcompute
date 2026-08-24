#[cfg(test)]
mod simple_components_test {
    use rcompute::components::event::MonitorEvent;
    use rcompute::components::orchestrator::Orchestrator;
    use rcompute::components::task::TaskEvent;
    use rcompute::components::worker::Worker;
    use rcompute::config::app_config::AppConfig;

    use std::sync::mpsc;

    pub fn get_config() -> AppConfig {
        let mut config: AppConfig = AppConfig::new();
        config.set_config(10, 3, 30, 20);
        return config;
    }

    #[test]
    fn instantiation() {
        let (monitor_tx, _monitor_rx) = mpsc::channel::<MonitorEvent>();
        let (task_tx, task_rx) = mpsc::channel::<TaskEvent>();
        let orchestrator = Orchestrator::from_config(1, monitor_tx.clone(), task_rx, get_config());
        assert_eq!(orchestrator.id, 1);
        assert_eq!(orchestrator.threshold, 3);
        assert_eq!(orchestrator.timeout, 30);
        assert_eq!(orchestrator.check_frequency, 20);

        let (monitor_tx, _monitor_rx) = mpsc::channel::<MonitorEvent>();
        let worker = Worker::new(1, task_tx.clone(), monitor_tx.clone());
        assert_eq!(worker.id, 1);
    }

    #[test]
    fn queuing() {
        let (monitor_tx, _monitor_rx) = mpsc::channel::<MonitorEvent>();
        let (_tx, rx) = mpsc::channel::<TaskEvent>();
        let mut orchestrator = Orchestrator::from_config(1, monitor_tx.clone(), rx, get_config());

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
