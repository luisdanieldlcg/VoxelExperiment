use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Hash, Deserialize, Default)]
pub struct Uid(pub u64);

impl From<u64> for Uid {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Uid> for u64 {
    fn from(value: Uid) -> Self {
        value.0
    }
}

impl std::fmt::Display for Uid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
