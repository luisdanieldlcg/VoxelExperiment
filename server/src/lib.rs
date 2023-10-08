use std::time::Duration;

use log::info;
pub struct Server {}

#[allow(clippy::new_without_default)]
impl Server {
    
    pub fn new() -> Self {
        info!("Server created.");
        Self {}
    }
    pub fn tick(&mut self, dt: Duration) {}
}
