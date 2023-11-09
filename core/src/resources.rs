use std::collections::HashMap;

use vek::Vec2;

use crate::{chunk::Chunk, uid::Uid};

/// This resource stores the time passed since the previous tick
#[derive(Default)]
pub struct DeltaTime(pub f32);

/// This is the time passed since the game started
#[derive(Default)]
pub struct ProgramTime(pub f64);

#[derive(Default)]
pub struct TerrainMap(pub HashMap<Vec2<i32>, Chunk>);

#[derive(Default)]
pub struct Ping(pub f64);

#[derive(Clone, Copy, Debug)]
pub enum GameMode {
    Client,
    Server,
    Singleplayer,
}


#[derive(Default)]
pub struct EntityMap {
    entities: HashMap<Uid, apecs::Entity>,
    next_uid: u64,
}

impl EntityMap {
    /// Inserts an entity into the map and returns the uid
    ///
    /// SERVER ONLY
    pub fn insert_entity(&mut self, entity: apecs::Entity) -> Uid {
        let uid = self.next_uid();
        self.entities.insert(uid, entity);
        uid
    }

    pub fn entity(&self, uid: Uid) -> Option<apecs::Entity> {
        self.entities.get(&uid).cloned()
    }

    pub fn remove(&mut self, uid: Uid) -> Option<apecs::Entity> {
        self.entities.remove(&uid)
    }

    fn next_uid(&mut self) -> Uid {
        let uid = self.next_uid;
        self.next_uid += 1;
        Uid(uid)
    }
}
