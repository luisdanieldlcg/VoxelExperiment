use common::clock::Clock;
use server::{config::ServerConfig, Server};

fn main() {
    common::init_logger("");

    let config = ServerConfig::toml();
    let mut server = Server::new(config).unwrap();
    let mut clock = Clock::default();

    loop {
        server.tick(clock.dt());
        clock.tick();
    }
}
