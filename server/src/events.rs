use core::{
    event::Events,
    resources::EntityMap,
    uid::Uid,
    SysResult,
};

use apecs::{ok, Write, *};

pub enum ServerEvent {
    ClientDisconnect(Uid),
}

#[derive(CanFetch)]
pub struct HandleServerEvents {
    events: Write<Events<ServerEvent>>,
    entities: Write<Entities>,
    entity_map: Write<EntityMap>,
}

pub fn handle_server_events(mut system: HandleServerEvents) -> SysResult {
    for event in &system.events.events {
        match event {
            ServerEvent::ClientDisconnect(uid) => {
                if let Some(entity) = system.entity_map.entity(*uid) {
                    system.entities.destroy(entity);
                    system.entity_map.remove(*uid);
                    log::info!("Client {} disconnected.", uid);
                } else {
                    log::error!(
                        "Entity with uid: {} was not found in the entity map. this is a bug",
                        uid
                    );
                }
            },
        }
    }
    ok()
}
