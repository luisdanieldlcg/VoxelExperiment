use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};

use core::{
    net::con::Connection,
    net::packet::{ClientPacket, ServerPacket},
    state::State,
};
use log::info;

pub struct Server {
    connection: Connection<ServerPacket, ClientPacket>,
    state: State,
}

#[allow(clippy::new_without_default)]
impl Server {
    pub fn new() -> Self {
        let local_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1234);
        let con = Connection::listen(local_addr).unwrap();
        log::info!("Server listening on {}", local_addr);
        let state = State::server().unwrap();
        Self {
            connection: con,
            state,
        }
    }

    pub fn tick(&mut self, dt: Duration) {
        if let Ok((packet, addr)) = self.connection.recv() {
            match packet {
                ClientPacket::Connect => {
                    info!("New client connected.");
                },
                ClientPacket::Disconnect => {
                    info!("Client disconnected.");
                },
            }
        }
    }
}
