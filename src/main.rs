use rcompute::components::orchestrator::Orchestrator;
use rcompute::components::worker::Worker;

fn main() {
    println!("Hello, world!");

    let orchestrator = Orchestrator::new(1, 10);
    println!("{}", orchestrator.to_string());

    let worker = Worker::new(1, 1);
    println!("{}", worker.to_string());
}
