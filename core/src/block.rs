#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BlockId {
    Air,
    Dirt,
    Grass,
    Stone,
}

impl BlockId {
    pub fn is_solid(self) -> bool {
        match self {
            BlockId::Air => false,
            BlockId::Dirt => true,
            BlockId::Grass => true,
            BlockId::Stone => true,
        }
    }

    pub fn is_air(self) -> bool {
        matches!(self, BlockId::Air)
    }

    pub fn id_str(self) -> &'static str {
        match self {
            BlockId::Air => "air",
            BlockId::Dirt => "dirt",
            BlockId::Grass => "grass",
            BlockId::Stone => "stone",
        }
    }
}

impl From<&str> for BlockId {
    fn from(s: &str) -> Self {
        match s {
            "air" => BlockId::Air,
            "dirt" => BlockId::Dirt,
            "grass" => BlockId::Grass,
            "stone" => BlockId::Stone,
            _ => panic!("Unknown block id: {}", s),
        }
    }
}
