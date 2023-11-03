use std::{net::SocketAddr, time::Duration};

use common::{
    net::{
        con::Connection,
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
        let state = State::client().unwrap();
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
