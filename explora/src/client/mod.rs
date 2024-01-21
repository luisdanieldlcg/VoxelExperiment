pub mod error;

use std::{io::ErrorKind, net::SocketAddr, time::Duration};

use common::{
    components::Pos,
    net::{
        connection::Connection,
        error::NetworkError,
        packet::{ClientPacket, PingPacket, ServerPacket},
    },
    resources::{Ping, ProgramTime, TerrainConfig, TerrainMap},
    state::State,
};
use log::info;

use self::error::Error;

pub struct Client {
    connection: Connection<ClientPacket, ServerPacket>,
    state: State,
    /// The last time we received a ping packet from the server
    last_ping_time: f64,
    packet_count: usize,
    last_chunk_request_time: f64,
}

impl Client {
    pub fn new(host: SocketAddr) -> Result<Self, Error> {
        let connection: Connection<ClientPacket, ServerPacket> = Connection::connect(host).unwrap();
        info!("Connecting to {}", host);
        connection.send(ClientPacket::Connect).unwrap();
        let mut state = State::client().expect("Failed to create client state");
        let instant = std::time::Instant::now();

        loop {
            match connection.recv() {
                Ok((packet, addr)) => {
                    log::info!("Received packet from {}: {:?}", addr, packet);
                    match packet {
                        ServerPacket::ClientSync { uid } => {
                            log::info!("Joined to game with uid {}", uid);
                            let entity = state.ecs_mut().entity();
                            entity.with_bundle((Pos::default(), uid));
                            break;
                        },
                        ServerPacket::Ping(_) => {},
                        _ => (),
                    }
                },
                // TODO: return errors instead of panicking
                Err(NetworkError::IOError(ErrorKind::WouldBlock)) => {
                    if instant.elapsed() > Duration::from_secs(5) {
                        return Err(Error::ServerTimeout);
                    }
                },
                Err(err) => {
                    return Err(Error::Other(format!(
                        "Failed to receive initial sync packet: {:?}",
                        err
                    )));
                },
            }
        }

        Ok(Self {
            connection,
            state,
            last_ping_time: 0.0,
            packet_count: 0,
            last_chunk_request_time: 0.0,
        })
    }

    pub fn tick(&mut self, dt: Duration) {
        self.state.tick(dt);

        let time = self.state.resource::<ProgramTime>();

        if time.0 - self.last_ping_time > 1.0 {
            self.send_packet(ClientPacket::Ping(PingPacket::Ping));
            self.last_ping_time = self.state.program_time();
        }

        if let Ok((packet, _)) = self.connection.recv() {
            match packet {
                ServerPacket::Ping(PingPacket::Ping) => {
                    // pong
                },
                ServerPacket::Ping(PingPacket::Pong) => {
                    // update ping
                    self.state_mut().resource_mut::<Ping>().0 =
                        self.state.program_time() - self.last_ping_time;
                },
                ServerPacket::ChunkUpdate { pos, data } => {
                    let chunk = common::chunk::decompress(&data);
                    let terrain = self.state.resource_mut::<TerrainMap>();
                    let old = terrain.chunks.insert(pos, chunk);
                    if let Some(old) = old {
                        log::warn!("Overwriting chunk at {:?} with new chunk", pos);
                    }
                    terrain.pending_chunks.remove(&pos);
                },
                _ => (),
            }
        }
        // this may run multiple times until the chunk arrives
        // so we'll throttle the chunk requests.
        if self.state.program_time() - self.last_chunk_request_time < 0.1 {
            return;
        }

        let terrain = self.state.resource::<TerrainMap>();
        for pending in &terrain.pending_chunks {
            if !terrain.chunks.contains_key(pending) {
                self.send_packet(ClientPacket::ChunkRequest(*pending));
                self.last_chunk_request_time = self.state.program_time();
                self.packet_count += 1;
            }
        }
    }

    pub fn send_packet(&self, packet: ClientPacket) {
        if let Err(e) = self.connection.send(packet) {
            log::error!("Failed to send packet: {:?}", e);
        }
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
