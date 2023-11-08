use serde::{Deserialize, Serialize};

use crate::uid::Uid;

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientPacket {
    Connect,
    Disconnect,
    Ping(PingPacket),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerPacket {
    ClientSync { uid: Uid },
    Ping(PingPacket),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PingPacket {
    Ping,
    Pong,
}
