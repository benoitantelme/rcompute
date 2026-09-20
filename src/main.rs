use rcompute::components::app::Application;
use rcompute::components::event::MonitorEvent;
use rcompute::components::monitor::Monitor;
use rcompute::config::app_config::AppConfig;
use std::sync::mpsc;

fn main() {
    let config = AppConfig::read_config();
    let (monitor_tx, monitor_rx) = mpsc::channel::<MonitorEvent>();
    let monitor = Monitor::new(1, monitor_rx, config.monitor_display);

    std::thread::spawn(move || monitor.run());

    let mut app = Application::new(1, config, monitor_tx);

    let a = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        .map(|row| row.to_vec())
        .to_vec();
    let b = [[9, 8, 7], [6, 5, 4], [3, 2, 1]]
        .map(|row| row.to_vec())
        .to_vec();
    let c = app.multiply(a, b);
    println!("SUMMA result: {c:?}");
    std::thread::sleep(std::time::Duration::from_millis(50));

    app.results
        .iter()
        .for_each(|(calc_id, result)| match result {
            Ok(matrix) => println!("Calculation {calc_id} result: {matrix:?}"),
            Err(err) => println!("Calculation {calc_id} error: {err:?}"),
        });
}
