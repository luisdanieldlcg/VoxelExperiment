use std::time::Duration;

pub struct State {}

#[allow(clippy::new_without_default)]
impl State {
    pub fn new() -> Self {
        Self {}
    }

    pub fn tick(&mut self, _: Duration) {}
}
