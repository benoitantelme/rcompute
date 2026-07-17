use figment::{
    Figment,
    providers::{Format, Toml},
};
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct AppConfig {
    pub orchestrator_workers_number: usize,
    pub orchestrator_workers_threshold: u32,
}

impl AppConfig {
    pub fn new() -> Self {
        let config_path = "src/conf/config.toml";
        let figment = Figment::from(Toml::file(config_path));
        let config: AppConfig = figment
            .extract()
            .expect(&("Failed to extract configuration from ".to_string() + config_path));

        Self {
            orchestrator_workers_number: config.orchestrator_workers_number,
            orchestrator_workers_threshold: config.orchestrator_workers_threshold,
        }
    }
}
