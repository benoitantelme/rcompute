#[cfg(test)]
mod config_test {
    use rcompute::config::app_config::AppConfig;

    #[test]
    fn instantiation() {
        let config: AppConfig = AppConfig::new();
        assert_eq!(config.workers_number, 10);
        assert_eq!(config.workers_threshold, 3);
        assert_eq!(config.timeout, 30 * 1000);
        assert_eq!(config.check_frequency, 20 * 1000);
    }
}
