#[cfg(test)]
mod config_test {
    use rcompute::config::app_config::AppConfig;

    #[test]
    fn instantiation() {
        let config: AppConfig = AppConfig::new();
        assert_eq!(config.orchestrator_workers_number, 10);
        assert_eq!(config.orchestrator_workers_threshold, 3);
    }
}
