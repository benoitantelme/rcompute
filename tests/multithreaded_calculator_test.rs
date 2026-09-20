use rcompute::components::app::to_vector;
use rcompute::components::event::MonitorEvent;
use rcompute::components::orchestrator::Orchestrator;
use rcompute::components::task::TaskEvent;
use rcompute::components::worker::Worker;
use std::sync::mpsc;

fn calculate_summa(a: Vec<Vec<u32>>, b: Vec<Vec<u32>>, matrix_size: usize) -> Vec<Vec<u32>> {
    let (monitor_sender, _monitor_receiver) = mpsc::channel::<MonitorEvent>();
    let (result_sender, result_receiver) = mpsc::channel::<TaskEvent>();
    let mut orchestrator =
        Orchestrator::new(1, monitor_sender.clone(), result_receiver, matrix_size);

    let size = (matrix_size * matrix_size) as u32;
    for worker_id in 1..=size {
        let (worker, work_sender) =
            Worker::with_work_channel(worker_id, result_sender.clone(), monitor_sender.clone());
        orchestrator.register_worker_channel(worker_id, work_sender);
        std::thread::spawn(move || worker.run());
    }

    orchestrator.multiply_summa(a, b).unwrap()
}

#[test]
fn instantiation() {
    let (monitor_sender, _monitor_receiver) = mpsc::channel::<MonitorEvent>();
    let (_result_sender, result_receiver) = mpsc::channel::<TaskEvent>();
    let orchestrator = Orchestrator::new(1, monitor_sender, result_receiver, 3);

    assert_eq!(orchestrator.matrix_size, 3);
    assert_eq!(orchestrator.result_matrix, [[0; 3]; 3]);
}

#[test]
fn instantiation_four() {
    let (monitor_sender, _monitor_receiver) = mpsc::channel::<MonitorEvent>();
    let (_result_sender, result_receiver) = mpsc::channel::<TaskEvent>();
    let orchestrator = Orchestrator::new(1, monitor_sender, result_receiver, 4);

    assert_eq!(orchestrator.matrix_size, 4);
    assert_eq!(orchestrator.result_matrix, [[0; 4]; 4]);
}

#[test]
fn calculate_ones() {
    assert_eq!(
        calculate_summa(
            to_vector([[1, 1, 1], [1, 1, 1], [1, 1, 1]]),
            to_vector([[1, 1, 1], [1, 1, 1], [1, 1, 1]]),
            3
        ),
        to_vector([[3, 3, 3], [3, 3, 3], [3, 3, 3]])
    );
}
#[test]
fn calculate_ones_four() {
    assert_eq!(
        calculate_summa(
            [[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]]
                .map(|row| row.to_vec())
                .to_vec(),
            [[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]]
                .map(|row| row.to_vec())
                .to_vec(),
            4
        ),
        [[4, 4, 4, 4], [4, 4, 4, 4], [4, 4, 4, 4], [4, 4, 4, 4]]
            .map(|row| row.to_vec())
            .to_vec()
    );
}

#[test]
fn identity() {
    let a = to_vector([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let identity = to_vector([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);

    assert_eq!(calculate_summa(a.clone(), identity.clone(), 3), a);
    assert_eq!(calculate_summa(identity.clone(), a.clone(), 3), a);
}

#[test]
fn zero() {
    let a = to_vector([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let zero = to_vector([[0, 0, 0], [0, 0, 0], [0, 0, 0]]);

    assert_eq!(calculate_summa(a.clone(), zero.clone(), 3), zero);
    assert_eq!(calculate_summa(zero.clone(), a.clone(), 3), zero);
}

#[test]
fn example() {
    let a = to_vector([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let b = to_vector([[9, 8, 7], [6, 5, 4], [3, 2, 1]]);
    assert_eq!(
        calculate_summa(a.clone(), b.clone(), 3),
        to_vector([[30, 24, 18], [84, 69, 54], [138, 114, 90]])
    );
}

#[test]
fn diagonal() {
    let a = to_vector([[2, 0, 0], [0, 3, 0], [0, 0, 4]]);
    let b = to_vector([[5, 0, 0], [0, 6, 0], [0, 0, 7]]);
    assert_eq!(
        calculate_summa(a.clone(), b.clone(), 3),
        to_vector([[10, 0, 0], [0, 18, 0], [0, 0, 28]])
    );
}

#[test]
fn column() {
    let a = to_vector([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let b = to_vector([[0, 0, 0], [0, 1, 0], [0, 0, 0]]);
    assert_eq!(
        calculate_summa(a.clone(), b.clone(), 3),
        to_vector([[0, 2, 0], [0, 5, 0], [0, 8, 0]])
    );
}

#[test]
fn random() {
    let a = to_vector([[2, 1, 3], [0, 4, 2], [5, 2, 1]]);
    let b = to_vector([[1, 3, 2], [4, 0, 1], [2, 5, 3]]);
    assert_eq!(
        calculate_summa(a.clone(), b.clone(), 3),
        to_vector([[12, 21, 14], [20, 10, 10], [15, 20, 15]])
    );
}
