#[cfg(test)]
mod config_test {
    use rcompute::config::app_config::AppConfig;

    #[test]
    fn instantiation() {
        let config: AppConfig = AppConfig::new();

        assert_eq!(config.workers_number, 0);
        assert_eq!(config.workers_threshold, 0);
        assert_eq!(config.timeout, 0);
        assert_eq!(config.check_frequency, 0);
    }

    #[test]
    fn read_config() {
        let config: AppConfig = AppConfig::read_config();

        assert_eq!(config.workers_number, 10);
        assert_eq!(config.workers_threshold, 3);
        assert_eq!(config.timeout, 30 * 1000);
        assert_eq!(config.check_frequency, 20 * 1000);
    }

     #[test]
    fn set_config() {
        let mut config: AppConfig = AppConfig::new();

        config.set_config(10, 3, 30, 20);
        assert_eq!(config.workers_number, 10);
        assert_eq!(config.workers_threshold, 3);
        assert_eq!(config.timeout, 30);
        assert_eq!(config.check_frequency, 20);
    }
}
