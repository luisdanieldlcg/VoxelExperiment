pub fn log_init() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init()
}
