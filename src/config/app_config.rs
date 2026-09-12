use figment::{
    Figment,
    providers::{Format, Toml},
};
use serde::Deserialize;

#[derive(Default, Debug, PartialEq, Deserialize)]
pub struct AppConfig {
    pub matrix_size: usize,
    pub timeout: u64,
    pub check_frequency: u64,
    pub monitor_display: bool,
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
            matrix_size: config.matrix_size,
            timeout: config.timeout,
            check_frequency: config.check_frequency,
            monitor_display: config.monitor_display,
        }
    }

    pub fn set_config(&mut self, matrix_size: usize, timeout: u64, check_frequency: u64, monitor_display: bool) {
        self.matrix_size = matrix_size;
        self.timeout = timeout;
        self.check_frequency = check_frequency;
        self.monitor_display = monitor_display;
    }
}
