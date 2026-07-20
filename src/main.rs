use rcompute::components::event::Event;
use rcompute::components::orchestrator::Orchestrator;
use rcompute::components::worker::Worker;
use rcompute::config::app_config::AppConfig;

use std::sync::mpsc;

fn main() {
    let config: AppConfig = AppConfig::new();
    let (tx, rx) = mpsc::channel::<Event>();
    let mut orchestrator = Orchestrator::new(
        1,
        rx,
        config.workers_number,
        config.workers_threshold,
        config.timeout,
        config.check_frequency,
    );
    println!("{}", orchestrator.to_string());
    orchestrator.initialise();

    std::thread::spawn(move || orchestrator.run());

    let worker = Worker::new(1, 1);
    println!("{}", worker.to_string());
}
