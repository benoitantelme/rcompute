use figment::{
    Figment,
    providers::{Format, Toml},
};
use serde::Deserialize;

#[derive(Default, Debug, PartialEq, Deserialize)]
pub struct AppConfig {
    pub workers_number: usize,
    pub workers_threshold: u32,
    pub timeout: u64,
    pub check_frequency: u64,
}

impl AppConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read_config() -> Self {
        let config_path = "src/conf/config.toml";
        let figment = Figment::from(Toml::file(config_path));
        let config: AppConfig = figment
            .extract()
            .expect(&("Failed to extract configuration from ".to_string() + config_path));

        Self {
            workers_number: config.workers_number,
            workers_threshold: config.workers_threshold,
            timeout: config.timeout,
            check_frequency: config.check_frequency,
        }
    }

    pub fn set_config(
        &mut self,
        workers_number: usize,
        workers_threshold: u32,
        timeout: u64,
        check_frequency: u64,
    ) {
        self.workers_number = workers_number;
        self.workers_threshold = workers_threshold;
        self.timeout = timeout;
        self.check_frequency = check_frequency;
    }
}
