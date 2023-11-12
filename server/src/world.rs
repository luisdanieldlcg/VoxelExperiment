use core::chunk::Chunk;

use noise::{BasicMulti, Perlin};
use vek::Vec2;

pub struct WorldGenerator {
    gen: BasicMulti<Perlin>,
}

impl WorldGenerator {
    pub fn new() -> Self {
        Self {
            gen: BasicMulti::new(884),
        }
    }

    pub fn generate_chunk(&self, offset: Vec2<i32>) -> Chunk {
        Chunk::generate(&self.gen, offset)
    }
}
