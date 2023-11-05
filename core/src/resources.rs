use std::collections::HashMap;

use vek::Vec2;

use crate::chunk::Chunk;

/// This resource stores the time passed since the previous tick
#[derive(Default)]
pub struct DeltaTime(pub f32);

#[derive(Default)]
pub struct TerrainMap(pub HashMap<Vec2<i32>, Chunk>);

#[derive(Clone, Copy, Debug)]
pub enum GameMode {
    Client,
    Server,
    Singleplayer,
}
