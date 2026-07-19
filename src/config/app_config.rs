use figment::{
    Figment,
    providers::{Format, Toml},
};
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct AppConfig {
    pub workers_number: usize,
    pub workers_threshold: u32,
    pub timeout: u32,
}

impl AppConfig {
    pub fn new() -> Self {
        let config_path = "src/conf/config.toml";
        let figment = Figment::from(Toml::file(config_path));
        let config: AppConfig = figment
            .extract()
            .expect(&("Failed to extract configuration from ".to_string() + config_path));

        Self {
            workers_number: config.workers_number,
            workers_threshold: config.workers_threshold,
            timeout: config.timeout,
        }
    }
}
