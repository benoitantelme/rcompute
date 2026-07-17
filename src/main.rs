use rcompute::components::orchestrator::Orchestrator;
use rcompute::components::worker::Worker;
use rcompute::config::app_config::AppConfig;


fn main() {
    let config: AppConfig = AppConfig::new();

    let orchestrator = Orchestrator::new(1, config.orchestrator_workers_number);
    println!("{}", orchestrator.to_string());

    let worker = Worker::new(1, 1);
    println!("{}", worker.to_string());
}
