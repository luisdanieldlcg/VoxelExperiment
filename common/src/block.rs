#[derive(Clone, Copy, Debug)]
pub enum BlockId {
    Air,
    Dirt,
}

impl BlockId {
    pub fn is_solid(self) -> bool {
        match self {
            BlockId::Air => false,
            BlockId::Dirt => true,
        }
    }
}
