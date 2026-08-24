#[cfg(test)]
mod simple_orchestrator_test {
    use rcompute::components::event::MonitorEvent;
    use rcompute::components::orchestrator::Orchestrator;
    use rcompute::components::task::Task::{TaskInput, TaskResult, TaskTimeout};
    use rcompute::components::task::TaskEvent;

    use std::sync::mpsc;

    #[test]
    fn init() {
        let (monitor_tx, _monitor_rx) = mpsc::channel::<MonitorEvent>();
        let (_tx, rx) = mpsc::channel::<TaskEvent>();
        let mut orchestrator = Orchestrator::new(1, monitor_tx.clone(), rx, 5, 3, 30, 30);

        orchestrator.initialise();
        assert_eq!(orchestrator.get_worker_queue_size(), 5);
    }

    #[test]
    fn threshold_test() {
        let (monitor_tx, _monitor_rx) = mpsc::channel::<MonitorEvent>();
        let (_tx, rx) = mpsc::channel::<TaskEvent>();
        let mut orchestrator = Orchestrator::new(1, monitor_tx.clone(), rx, 5, 3, 30, 30);

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
        let (monitor_tx, _monitor_rx) = mpsc::channel::<MonitorEvent>();
        let (_tx, rx) = mpsc::channel::<TaskEvent>();
        let mut orchestrator = Orchestrator::new(1, monitor_tx.clone(), rx, 5, 3, 30, 30);

        orchestrator.initialise();

        for _n in 1..6 {
            orchestrator.pull_worker();
        }

        orchestrator.pull_worker();
    }

    #[test]
    fn check_available_workers() {
        let (monitor_tx, _monitor_rx) = mpsc::channel::<MonitorEvent>();
        let (_tx, rx) = mpsc::channel::<TaskEvent>();
        let mut orchestrator = Orchestrator::new(1, monitor_tx.clone(), rx, 5, 3, 30, 30);

        assert_eq!(orchestrator.available_workers.len(), 0);
        orchestrator.initialise();
        assert_eq!(orchestrator.available_workers.len(), 5);

        for _n in 1..3 {
            orchestrator.pull_worker();
        }
        assert_eq!(orchestrator.available_workers.len(), 3);
    }

    #[test]
    fn task_lifecycle() {
        let (monitor_tx, _monitor_rx) = mpsc::channel::<MonitorEvent>();
        let (_task_tx, task_rx) = mpsc::channel::<TaskEvent>();

        let mut orchestrator = Orchestrator::new(1, monitor_tx.clone(), task_rx, 5, 3, 30, 30);
        orchestrator.initialise();

        assert_eq!(orchestrator.closed_tasks.len(), 0);
        assert_eq!(orchestrator.open_tasks.len(), 0);
        assert_eq!(orchestrator.failed_tasks.len(), 0);

        orchestrator.handle_task_input(1, 1, 41);

        assert_eq!(orchestrator.closed_tasks.len(), 0);
        assert_eq!(orchestrator.open_tasks.len(), 1);
        assert_eq!(orchestrator.failed_tasks.len(), 0);

        orchestrator.handle_task_result(1, 1, 42);

        assert_eq!(orchestrator.closed_tasks.len(), 1);
        assert_eq!(orchestrator.open_tasks.len(), 0);
        assert_eq!(orchestrator.failed_tasks.len(), 0);

        orchestrator.handle_task_input(1, 2, 41);

        assert_eq!(orchestrator.closed_tasks.len(), 1);
        assert_eq!(orchestrator.open_tasks.len(), 1);
        assert_eq!(orchestrator.failed_tasks.len(), 0);
    }
}
