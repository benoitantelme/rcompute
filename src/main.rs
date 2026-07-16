mod components;
use crate::components::orchestrator::Orchestrator;
use crate::components::worker::Worker;

fn main() {
    println!("Hello, world!");

    let orchestrator = Orchestrator::new(1);
    println!("{}", orchestrator.to_string());

    let worker = Worker::new(1, 1, orchestrator);
    println!("{}", worker.to_string());
}
