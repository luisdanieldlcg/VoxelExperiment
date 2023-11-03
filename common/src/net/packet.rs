use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientPacket {
    Connect,
    Disconnect
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerPacket {}
