use std::time::Duration;

pub struct State {}

impl State {

    pub fn new() -> Self {
        Self {}
    }

    pub fn tick(&mut self, _: Duration) {}
}