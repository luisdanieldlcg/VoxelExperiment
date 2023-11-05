#[derive(Debug)]
pub enum NetworkError {
    ConnectionFailed,
    SocketBindError,
    DeserializeError(bincode::Error),
    IOError(std::io::ErrorKind),
}
