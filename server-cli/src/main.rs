use server::Server;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .init();
    let mut server = Server::new();

    log::info!("Test info");
    log::debug!("Test debug");
    log::warn!("Test warn");
    log::error!("Test error");
    log::trace!("Test trace");

    loop {
        server.tick(std::time::Duration::ZERO);
    }
}
