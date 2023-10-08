fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .init();

    log::info!("Test info");
    log::debug!("Test debug");
    log::warn!("Test warn");
    log::error!("Test error");
    log::trace!("Test trace");
}
