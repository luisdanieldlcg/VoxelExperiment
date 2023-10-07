use std::time::Duration;

pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Self {}
    }
    pub fn tick(&mut self, dt: Duration) {}
}

