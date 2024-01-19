pub mod error;

use std::{io::ErrorKind, net::SocketAddr, time::Duration};

use crate::render::resources::TerrainRender;
use apecs::{ok, CanFetch, Query, ShouldContinue, Write};
use common::{
    components::Pos,
    net::{
        connection::Connection,
        error::NetworkError,
        packet::{ClientPacket, PingPacket, ServerPacket},
    },
    resources::{Ping, ProgramTime, TerrainMap},
    state::State,
};
use log::info;
use vek::Vec2;

use crate::camera::Camera;

use self::error::Error;

pub struct Client {
    connection: Connection<ClientPacket, ServerPacket>,
    state: State,
    /// The last time we received a ping packet from the server
    last_ping_time: f64,
}

impl Client {
    pub fn new(host: SocketAddr, aspect: f32) -> Result<Self, Error> {
        let connection: Connection<ClientPacket, ServerPacket> = Connection::connect(host).unwrap();
        info!("Connecting to {}", host);
        connection.send(ClientPacket::Connect).unwrap();
        let mut state = State::client().expect("Failed to create client state");
        state
            .ecs_mut()
            .with_system("chunk_load", chunk_load_system)
            .unwrap();
        let instant = std::time::Instant::now();

        loop {
            match connection.recv() {
                Ok((packet, addr)) => {
                    log::info!("Received packet from {}: {:?}", addr, packet);
                    match packet {
                        ServerPacket::ClientSync { uid } => {
                            log::info!("Joined to game with uid {}", uid);
                            let entity = state.ecs_mut().entity();
                            let mut camera = Camera::new(aspect);
                            camera.rotate(0.0, 0.0);
                            entity.with_bundle((camera, Pos::default(), uid));
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
        })
    }

    pub fn tick(&mut self, dt: Duration) {
        self.state.tick(dt);

        let time = self.state.resource::<ProgramTime>();

        if time.0 - self.last_ping_time > 2.0 {
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
                    let old = self
                        .state
                        .resource_mut::<TerrainMap>()
                        .chunks
                        .insert(pos, chunk);
                    if let Some(old) = old {
                        log::warn!("Overwriting chunk at {:?} with new chunk", pos);
                    }
                    self.state
                        .resource_mut::<TerrainMap>()
                        .pending_chunks
                        .remove(&pos);
                },
                _ => (),
            }
        }

        let terrain = self.state.resource::<TerrainMap>();
        // this may run multiple times per tick until the chunk arrives
        // so we need to check if the chunk is already requested
        for pending in &terrain.pending_chunks {
            if !terrain.chunks.contains_key(pending) {
                self.send_packet(ClientPacket::ChunkRequest(*pending));
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

use apecs::*;
#[derive(CanFetch)]
pub struct ChunkLoadSystem {
    terrain: Write<TerrainMap>,
    camera: Query<&'static Camera>,
    terrain_render: Write<TerrainRender>,
}

pub fn chunk_load_system(mut system: ChunkLoadSystem) -> apecs::anyhow::Result<ShouldContinue> {
    if let Some(camera) = system.camera.query().find_one(0) {
        let camera_pos = camera.pos();
        let player_chunk_pos = Vec2::new(
            (camera_pos.x / 16.0).floor() as i32,
            (camera_pos.z / 16.0).floor() as i32,
        );
        let render_dist = 8;

        let mut chunks_to_remove = Vec::with_capacity(system.terrain.chunks.len());
        for (pos, _) in system.terrain.chunks.iter() {
            let dist = Vec2::new(
                (pos.x - player_chunk_pos.x).abs(),
                (pos.y - player_chunk_pos.y).abs(),
            );

            if dist.x > render_dist || dist.y > render_dist {
                chunks_to_remove.push(*pos);
            }
        }

        for chunk_pos in chunks_to_remove {
            system.terrain.chunks.remove(&chunk_pos);
            system.terrain_render.chunks.remove(&chunk_pos);
            log::info!("Unloading chunk at {:?}", chunk_pos);
        }

        // load chunks
        let start_x = player_chunk_pos.x - render_dist;
        let start_z = player_chunk_pos.y - render_dist;
        let end_x = player_chunk_pos.x + render_dist;
        let end_z = player_chunk_pos.y + render_dist;

        for x in start_x..=end_x {
            for z in start_z..=end_z {
                let chunk_pos = Vec2::new(x, z);
                if !system.terrain.chunks.contains_key(&chunk_pos)
                    && !system.terrain.pending_chunks.contains(&chunk_pos)
                {
                    system.terrain.pending_chunks.insert(chunk_pos);
                    log::info!("Loading chunk at {:?}", chunk_pos);
                }
            }
        }
    }
    ok()
}
impl Drop for Client {
    fn drop(&mut self) {
        self.connection.send(ClientPacket::Disconnect).unwrap();
    }
}
