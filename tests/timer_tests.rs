#[cfg(test)]
mod timer_test {
    use rcompute::components::event::Event;
    use rcompute::components::orchestrator::Orchestrator;
    use rcompute::components::timer::Deadline;

    use std::sync::mpsc;
    use std::{thread, time};

    #[test]
    fn check_expiration() {
        let deadline = Deadline::new(1, 100);
        assert_eq!(deadline.is_expired(), false);
        thread::sleep(time::Duration::from_millis(10));
        assert_eq!(deadline.is_expired(), false);
        thread::sleep(time::Duration::from_millis(100));
        assert_eq!(deadline.is_expired(), true);
    }

    #[test]
    fn check_values() {
        let deadline = Deadline::new(1, 100);
        assert_eq!(deadline.task_id, 1);
        assert!(deadline.when < time::SystemTime::now() + time::Duration::from_millis(100));
    }

    #[test]
    fn check_ordering() {
        let (tx, rx) = mpsc::channel::<Event>();
        let mut orchestrator = Orchestrator::new(1, rx, 5, 3, 30, 30);
        orchestrator.initialise();

        for n in 1..5 {
            let deadline = Deadline::new(n, n as u64 * 100);
            orchestrator.deadlines.push(deadline);
        }

        for n in 1..5 {
            let deadline = orchestrator.deadlines.pop().unwrap();
            assert_eq!(deadline.task_id, n);
        }
    }

    #[test]
    fn orchestrator_timeouts() {
        let (tx, rx) = mpsc::channel::<Event>();
        // TODO: This test is currently failing because the orchestrator is not handling timeouts correctly. We need to fix the timeout 
        // handling in the orchestrator before this test can pass. This will be done later with timeouts messages
        let mut orchestrator = Orchestrator::new(1, rx, 5, 3, 30, 30);
        orchestrator.initialise();
        std::thread::spawn(move || orchestrator.run());

        // assert_eq!(some timeouts);
    }
}
