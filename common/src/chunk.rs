use noise::{NoiseFn, Perlin};
use vek::{Vec2, Vec3};

use crate::block::BlockId;

pub struct Chunk {
    blocks: [BlockId; 16 * 256 * 16],
}

use rayon::{
    iter::{IndexedParallelIterator, IntoParallelRefMutIterator},
    prelude::ParallelIterator,
};

pub fn compute_height(generator: &noise::BasicMulti<Perlin>, world_x: f64, world_z: f64) -> i32 {
    let height = generator.get([world_x, world_z]);
    // Noise values are in range [-1, 1]
    // then adding 1 will transform them to [0, 2]
    // Dividing each of the new values by 2 will re-scale them to the final range [0,1]
    let height = height + 1.0 / 2.0;
    // Now we scale it to appropiate chunk height
    (height * Chunk::SIZE.y as f64) as i32
}

impl Chunk {
    pub const SIZE: Vec3<usize> = Vec3::new(16, 256, 16);

    pub fn flat(id: BlockId) -> Self {
        Self {
            blocks: [id; Self::SIZE.x * Self::SIZE.y * Self::SIZE.z],
        }
    }

    pub fn generate(generator: &noise::BasicMulti<Perlin>, offset: Vec2<i32>) -> Self {
        let world_x = (offset.x * Self::SIZE.x as i32) as f64;
        let world_z = (offset.y * Self::SIZE.z as i32) as f64;

        let mut blocks = [BlockId::Air; 16 * 256 * 16];

        blocks.par_iter_mut().enumerate().for_each(|(id, block)| {
            let x = id % Self::SIZE.x;
            let y = (id / Self::SIZE.x) % Self::SIZE.y;
            let z = (id / (Self::SIZE.x * Self::SIZE.y)) % Self::SIZE.z;

            let noise_x = (world_x + x as f64) / 600.0;
            let noise_z = (world_z + z as f64) / 600.0;
            let height = compute_height(generator, noise_x, noise_z);

            let offset = 700.0;
            let noise_x = (world_x + x as f64) / offset;
            let noise_z = (world_z + z as f64) / offset;
            let stone_height = compute_height(generator, noise_x, noise_z);
            let stone_height = ((stone_height as f32) * 0.7) as i32;

            let y = y as i32;

            if y == height {
                *block = BlockId::Grass
            } else if y < height && y > stone_height {
                *block = BlockId::Dirt
            } else if y < stone_height {
                *block = BlockId::Stone
            } else {
                *block = BlockId::Air
            }
        });

        Self { blocks }
    }

    pub fn index_of(pos: Vec3<i32>) -> Option<usize> {
        if pos.is_any_negative() {
            return None;
        }
        let pos = pos.map(|x| x as usize);

        if pos.x >= Self::SIZE.x || pos.y >= Self::SIZE.y || pos.z >= Self::SIZE.z {
            None
        } else {
            Some(pos.x + pos.y * Self::SIZE.x + pos.z * Self::SIZE.x * Self::SIZE.y)
        }
    }

    pub fn get(&self, pos: Vec3<i32>) -> Option<BlockId> {
        Self::index_of(pos).map(|idx| self.blocks[idx])
    }

    pub fn within_bounds(pos: Vec3<i32>) -> bool {
        !Self::out_of_bounds(pos)
    }

    pub fn out_of_bounds(pos: Vec3<i32>) -> bool {
        pos.is_any_negative()
            || pos.x >= Self::SIZE.x as i32
            || pos.y >= Self::SIZE.y as i32
            || pos.z >= Self::SIZE.z as i32
    }

    pub fn iter(&self) -> ChunkIter {
        ChunkIter {
            index: 0,
            size: Self::SIZE.map(|x| x as u32),
        }
    }
}

pub fn compress(c: &Chunk) -> Vec<(BlockId, u32)> {
    let mut compressed = Vec::with_capacity(600);
    let mut current_block = c.blocks[0];
    let mut count = 1;
    for &block in c.blocks.iter().skip(1) {
        // run length encoding
        if block == current_block {
            count += 1;
        } else {
            compressed.push((current_block, count));
            current_block = block;
            count = 1;
        }
    }
    // Don't forget to add the last run
    compressed.push((current_block, count));
    compressed
}

pub fn decompress(compressed: &[(BlockId, u32)]) -> Chunk {
    let mut blocks = [BlockId::Air; 16 * 256 * 16];
    let mut index = 0;
    for (block, count) in compressed {
        for _ in 0..*count {
            blocks[index] = *block;
            index += 1;
        }
    }
    Chunk { blocks }
}

pub struct ChunkIter {
    index: u32,
    size: Vec3<u32>,
}

impl Iterator for ChunkIter {
    type Item = Vec3<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.size.product() {
            return None;
        }
        let x = self.index % self.size.x;
        let y = (self.index / self.size.x) % self.size.y;
        let z = self.index / (self.size.x * self.size.y);

        self.index += 1;
        Some(Vec3::new(x, y, z).map(|f| f as i32))
    }
}

#[cfg(test)]
mod tests {
    use vek::Vec3;

    use crate::{
        block::BlockId,
        chunk::{compress, Chunk},
    };

    #[test]
    pub fn chunk_iter_works() {
        let chunk = Chunk::flat(BlockId::Air);
        let mut count = 0;

        for pos in chunk.iter() {
            assert!(Chunk::within_bounds(pos));
            count += 1;
        }

        assert_eq!(count, 16 * 256 * 16);
    }
    #[test]
    pub fn is_chunk_pos_out_of_bounds() {
        assert!(Chunk::out_of_bounds(Vec3::new(-1, 0, 0)));
        assert!(Chunk::out_of_bounds(Vec3::new(0, -1, 0)));
        assert!(Chunk::out_of_bounds(Vec3::new(0, 0, -1)));
        assert!(Chunk::out_of_bounds(Vec3::new(16, 0, 0)));
        assert!(Chunk::out_of_bounds(Vec3::new(0, 256, 0)));
        assert!(Chunk::out_of_bounds(Vec3::new(0, 0, 16)));
        assert!(!Chunk::out_of_bounds(Vec3::new(15, 255, 15)));
    }

    #[test]
    pub fn chunk_compression_test() {
        let chunk = Chunk::flat(BlockId::Dirt);
        let compressed = compress(&chunk);
        assert_eq!(compressed.len(), 1);
        assert_eq!(compressed[0], (BlockId::Dirt, 16 * 256 * 16));
    }
}
