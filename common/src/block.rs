use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BlockId {
    Air,
    Dirt,
    Grass,
    Stone,
}

impl BlockId {
    pub const fn is_air(self) -> bool {
        matches!(self, BlockId::Air)
    }

    pub const fn is_solid(self) -> bool {
        !self.is_air()
    }
}

impl From<&str> for BlockId {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "air" => BlockId::Air,
            "dirt" => BlockId::Dirt,
            "grass" => BlockId::Grass,
            "stone" => BlockId::Stone,
            _ => panic!("Unknown block id: {}", s),
        }
    }
}
