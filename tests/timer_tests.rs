#[cfg(test)]
mod timer_test {
    use rcompute::components::timer::Deadline;
    use std::{thread, time};

    #[test]
    fn check_expiration() {
        let deadline = Deadline::new(1, 1, 100);
        assert!(!deadline.is_expired());
        thread::sleep(time::Duration::from_millis(110));
        assert!(deadline.is_expired());
    }

    #[test]
    fn check_values() {
        let deadline = Deadline::new(1, 1, 100);
        assert_eq!(deadline.task_id, 1);
        assert_eq!(deadline.worker_id, 1);
    }
}
