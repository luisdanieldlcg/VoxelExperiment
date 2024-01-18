use common::clock::Clock;
use server::{config::ServerConfig, Server};

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let config = ServerConfig::toml();
    let mut server = Server::new(config).unwrap();
    let mut clock = Clock::default();

    loop {
        server.tick(clock.dt());
        clock.tick();
    }
}
