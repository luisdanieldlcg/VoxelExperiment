use std::{io::ErrorKind, net::SocketAddr, panic, time::Duration};

use core::{
    net::{
        con::Connection,
        error::NetworkError,
        packet::{ClientPacket, ServerPacket},
    },
    state::State,
};
use log::info;

pub struct Client {
    connection: Connection<ClientPacket, ServerPacket>,
    state: State,
}

impl Client {
    pub fn new(host: SocketAddr) -> Self {
        let connection: Connection<ClientPacket, ServerPacket> = Connection::connect(host).unwrap();
        info!("Connecting to {}", host);
        connection.send(ClientPacket::Connect).unwrap();
        let state = State::client().expect("Failed to create client state");

        let instant = std::time::Instant::now();

        loop {
            info!("Waiting for sync packet");
            match connection.recv() {
                Ok((packet, addr)) => {
                    log::info!("Received packet from {}: {:?}", addr, packet);
                    match packet {
                        ServerPacket::ClientSync { uid } => {
                            log::info!("Joined to game with uid {}", uid);
                            break;
                        },
                        ServerPacket::Ping(packet) => {},
                    }
                },
                // TODO: return errors instead of panicking
                Err(NetworkError::IOError(ErrorKind::WouldBlock)) => {
                    if instant.elapsed() > Duration::from_secs(5) {
                        log::error!("Failed to receive sync packet. Timeout");
                        break;
                    }
                },
                Err(err) => {
                    log::error!("Failed to receive sync packet: {:?}", err);
                    break;
                },
            }
        }

        Self { connection, state }
    }

    pub fn tick(&mut self, dt: Duration) {
        self.state.tick(dt);
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.connection.send(ClientPacket::Disconnect).unwrap();
    }
}
