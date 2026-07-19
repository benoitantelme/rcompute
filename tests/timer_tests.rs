// TODO: write tests to check if the timer works properly, checking if we have timeouts and calling the orchestrator properly

use rcompute::components::timer::Deadline;

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
