#[derive(Debug)]
pub enum Error {
    ServerTimeout,
    Other(String),
}
