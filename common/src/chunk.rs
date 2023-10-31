use vek::{Vec2, Vec3};

use crate::block::BlockId;

pub struct Chunk {
    blocks: Vec<BlockId>,
}

impl Chunk {
    pub const SIZE: Vec3<usize> = Vec3::new(16, 256, 16);

    pub fn generate(_: Vec2<i32>) -> Self {
        let mut blocks = vec![BlockId::Air; Self::SIZE.product()];
        for x in 0..Self::SIZE.x {
            for y in 0..Self::SIZE.y {
                for z in 0..Self::SIZE.z {
                    let local_pos = Vec3::new(x, y, z).map(|x| x as i32);
                    let index = match Self::index_of(local_pos) {
                        Some(i) => i,
                        None => continue,
                    };
                    blocks[index] = BlockId::Dirt;
                }
            }
        }
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

    pub fn is_within_bounds(pos: Vec3<i32>) -> bool {
        pos.x >= 0
            && pos.x < Self::SIZE.x as i32
            && pos.y >= 0
            && pos.y < Self::SIZE.y as i32
            && pos.z >= 0
            && pos.z < Self::SIZE.z as i32
    }

    pub fn iter(&self) -> ChunkIter {
        ChunkIter {
            index: 0,
            size: Self::SIZE.map(|x| x as u32),
        }
    }
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
