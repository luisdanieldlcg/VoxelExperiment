use serde::{de::DeserializeOwned, Serialize};
use std::net::{SocketAddr, UdpSocket};

use super::{error::NetworkError, socket};

/// Represents a connection that can either send or receive packets.
///
/// Generic Types:
/// - `S` stands for Send, i.e the packet type that is sent
/// - `R` stands for Receive, i.e the packet type that is received
pub struct Connection<S: Serialize, R: DeserializeOwned> {
    /// The internal socket that is used to send and receive packets
    pub(crate) socket: UdpSocket,
    /// A marker to let the compiler know that it should allow the generic types.
    _marker: std::marker::PhantomData<(S, R)>,
}

impl<S: Serialize, R: DeserializeOwned> Connection<S, R> {
    /// Connect to a remote host.
    ///
    /// This will bind a UDP socket to a random port and connect it to the remote host.
    pub fn connect(remote_addr: SocketAddr) -> Result<Self, NetworkError> {
        let socket = Self::bind(SocketAddr::from(([0, 0, 0, 0], 0)))?;
        socket
            .connect(remote_addr)
            .map_err(|_| NetworkError::ConnectionFailed)?;
        Ok(Self {
            socket,
            _marker: std::marker::PhantomData,
        })
    }

    /// Listen for incoming connections on a local address.
    ///
    /// This will bind a UDP socket to the local address
    /// and will be able to receive packets from any remote host.
    pub fn listen(local_addr: SocketAddr) -> Result<Self, NetworkError> {
        let socket = Self::bind(local_addr)?;
        Ok(Self {
            socket,
            _marker: std::marker::PhantomData,
        })
    }

    /// Send a packet to the remote host that this connection was made to.
    ///
    /// Fails if the socket is not connected.
    pub fn send(&self, packet: S) -> Result<(), NetworkError> {
        let packet = Self::serialize(&packet);
        self.socket
            .send(&packet)
            .map_err(|e| NetworkError::IOError(e.kind()))?;
        Ok(())
    }

    pub fn send_to(&self, packet: S, addr: SocketAddr) -> Result<(), NetworkError> {
        let packet = Self::serialize(&packet);
        self.socket
            .send_to(&packet, addr)
            .map_err(|e| NetworkError::IOError(e.kind()))?;
        Ok(())
    }

    /// Receive a packet. This will not block, if there is no packet it will return an error.
    pub fn recv(&self) -> Result<(R, SocketAddr), NetworkError> {
        let mut buf = [0; 2 << 14];
        match self.socket.recv_from(&mut buf) {
            Ok((len, addr)) => Self::deserialize(&buf[..len]).map(|p| (p, addr)),
            Err(e) => Err(NetworkError::IOError(e.kind())),
        }
    }

    fn bind(addr: SocketAddr) -> Result<UdpSocket, NetworkError> {
        socket::bind_udp_socket(addr).map_err(|_| NetworkError::SocketBindError)
    }

    fn serialize(packet: &S) -> Vec<u8> {
        let writer = bincode::serialize(packet).expect("Failed to serialize packet");
        lz4_compress::compress(&writer)
    }

    fn deserialize(packet: &[u8]) -> Result<R, NetworkError> {
        let buf = lz4_compress::decompress(packet).expect("decompression failed");
        match bincode::deserialize::<R>(buf.as_slice()) {
            Ok(t) => Ok(t),
            Err(e) => Err(NetworkError::DeserializeError(e)),
        }
    }
}

#[cfg(test)]
pub mod tests {

    pub fn create_client_server() {}
}
