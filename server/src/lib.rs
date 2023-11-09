pub mod config;
pub mod events;

use std::{net::SocketAddr, time::Duration};

use apecs::CanFetch;
use config::ServerConfig;
use core::{
    event::Events,
    net::con::Connection,
    net::packet::{ClientPacket, PingPacket, ServerPacket},
    resources::{EntityMap, ProgramTime},
    state::State,
    uid::Uid,
    SysResult,
};
use log::info;

type ServerConnection = Connection<ServerPacket, ClientPacket>;

pub struct RemoteClient {
    addr: SocketAddr,
    last_ping: f64,
}

pub struct Server {
    state: State,
}

#[allow(clippy::new_without_default)]
impl Server {
    pub fn new(config: ServerConfig) -> anyhow::Result<Self> {
        let addr = format!("{}:{}", config.host, config.port)
            .parse::<SocketAddr>()
            .expect("Failed to parse server address");
        let con: ServerConnection = Connection::listen(addr).unwrap();
        log::info!("Server listening on {}", addr);
        let mut state = State::server().unwrap();

        state
            .ecs_mut()
            .with_resource(con)?
            .with_resource(config)?
            .with_system_with_dependencies(
                "handle_incoming_packets",
                handle_incoming_packets,
                &[],
                &[],
            )?
            .with_system_with_dependencies(
                "handle_client_ping",
                handle_client_ping,
                &[],
                &["handle_server_events"],
            )?
            .with_system_with_dependencies(
                "handle_server_events",
                events::handle_server_events,
                &[],
                &["server_events-update"],
            )?;

        state.with_event::<ServerEvent>("server_events");

        core::state::print_system_schedule(state.ecs_mut());

        Ok(Self { state })
    }

    pub fn tick(&mut self, dt: Duration) {
        self.state.tick(dt);
    }
}

use apecs::*;

use crate::events::ServerEvent;

#[derive(CanFetch)]
pub struct HandleIncomingPacketsSystem {
    connection: Read<ServerConnection, NoDefault>,
    entities: Write<Entities>,
    entity_map: Write<EntityMap>,
    global_time: Read<ProgramTime>,
}

pub fn handle_incoming_packets(mut sys: HandleIncomingPacketsSystem) -> SysResult {
    if let Ok((packet, addr)) = sys.connection.recv() {
        match packet {
            ClientPacket::Connect => {
                let mut client = sys.entities.create();
                let uid = sys.entity_map.insert_entity(client.clone());

                let remote = RemoteClient {
                    addr,
                    last_ping: sys.global_time.0,
                };

                client.insert_bundle((uid, remote));

                let sync_packet = ServerPacket::ClientSync { uid };

                if let Err(e) = sys.connection.send_to(sync_packet, addr) {
                    log::error!("Failed to send sync packet to client: {:?}", e);
                }
                info!("New client connected.");
            },
            ClientPacket::Disconnect => {
                // TODO: send server event
            },
            ClientPacket::Ping(packet) => match packet {
                PingPacket::Ping => {
                    if let Err(error) = sys
                        .connection
                        .send_to(ServerPacket::Ping(PingPacket::Pong), addr)
                    {
                        log::error!("Failed to send ping packet to client: {:?}", error);
                    }
                },
                PingPacket::Pong => {},
            },
        }
    }
    ok()
}

#[derive(CanFetch)]
pub struct HandleClientPing {
    clients: Query<(&'static mut Uid, &'static mut RemoteClient)>,
    global_time: Read<ProgramTime>,
    events: Write<Events<ServerEvent>>,
    config: Read<ServerConfig, NoDefault>,
}

pub fn handle_client_ping(mut sys: HandleClientPing) -> SysResult {
    let mut query = sys.clients.query();

    for (uid, client) in query.iter_mut() {
        let delta = sys.global_time.0 - client.last_ping;

        if delta > sys.config.timeout as f64 {
            log::info!("Client {} timed out.", uid.0);
            sys.events.send(ServerEvent::ClientDisconnect(**uid));
        }
        // TODO: maybe try pinging if timeout is getting close ?
    }
    ok()
}
