use core::clock::Clock;
use server::Server;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    log::info!("Test info");
    log::debug!("Test debug");
    log::warn!("Test warn");
    log::error!("Test error");
    log::trace!("Test trace");

    let mut server = Server::new().unwrap();
    let mut clock = Clock::default();

    loop {
        server.tick(clock.dt());
        clock.tick();
    }
}
