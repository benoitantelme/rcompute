use rcompute::components::event::MonitorEvent;
use rcompute::components::task::{Task, TaskEvent};
use rcompute::components::worker::Worker;
use std::sync::mpsc;
use std::time::Duration;

#[test]
fn worker_processes_multiply_and_returns_partial_result() {
    let (result_sender, result_receiver) = mpsc::channel();
    let (monitor_sender, _monitor_receiver) = mpsc::channel::<MonitorEvent>();
    let (worker, work_sender) = Worker::with_work_channel(7, result_sender, monitor_sender);
    std::thread::spawn(move || worker.run());

    work_sender
        .send(TaskEvent::new(7, 10, Task::Multiply { a: 6, b: 7, k: 3 }))
        .unwrap();

    let result = result_receiver
        .recv_timeout(Duration::from_secs(1))
        .unwrap();
    assert_eq!(result.worker_id, 7);
    assert_eq!(result.task_id, 10);
    assert_eq!(result.task, Task::PartialResult { value: 42, k: 3 });
}
