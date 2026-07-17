use rcompute::components::orchestrator::Orchestrator;
use rcompute::components::worker::Worker;
use rcompute::config::app_config::AppConfig;


fn main() {
    let config: AppConfig = AppConfig::new();

    let mut orchestrator = Orchestrator::new(1, config.orchestrator_workers_number, config.orchestrator_workers_threshold);
    println!("{}", orchestrator.to_string());


    orchestrator.initialise();
    
    let worker = Worker::new(1, 1);
    println!("{}", worker.to_string());
}
