use core::{event::Events, uid::Uid, SysResult};

use apecs::{ok, Write, *};

pub enum ServerEvent {
    ClientDisconnect(Uid),
}

#[derive(CanFetch)]
pub struct HandleServerEvents {
    events: Write<Events<ServerEvent>>,
    entities: Write<Entities>,
}

pub fn handle_server_events(system: HandleServerEvents) -> SysResult {
    for event in &system.events.events {
        match event {
            ServerEvent::ClientDisconnect(uid) => {
                let entity = system.entities.hydrate(uid.0 as usize);
                if let Some(entity) = entity {
                    system.entities.destroy(entity);
                    // TODO: update entity map
                    log::info!("Client {} disconnected.", uid.0);
                } else {
                    log::error!("entity not found.");
                }
            },
        }
    }
    ok()
}
