use common::clock::Clock;

use std::{net::SocketAddr, sync::mpsc};

use server::{config::ServerConfig, Server};

pub struct Singleplayer {
    init_receiver: mpsc::Receiver<SocketAddr>,
}

impl Singleplayer {
    pub fn init() -> Self {
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let config = ServerConfig::toml();
            let addr = format!("{}:{}", config.host, config.port)
                .parse::<SocketAddr>()
                .expect("Failed to parse server address");
            match server::Server::new(config) {
                Ok(server) => {
                    if let Err(e) = tx.send(addr) {
                        log::error!("{:?}", e);
                    }
                    self::run_singleplayer_server(server);
                },
                Err(_) => {
                    panic!("Failed to initialize singleplayer server.");
                },
            };
        });

        Self { init_receiver: rx }
    }

    pub fn wait_for_init(&self) -> SocketAddr {
        self.init_receiver
            .recv()
            .expect("Failed to send initialization message")
    }
}

pub fn run_singleplayer_server(mut server: Server) {
    log::info!("Starting singleplayer server...");
    let mut clock = Clock::default();
    loop {
        clock.tick();
        server.tick(clock.dt());
    }
}
