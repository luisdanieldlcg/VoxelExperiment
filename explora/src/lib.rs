use common::{clock::Clock, state::State};

pub mod error;
pub mod input;
pub mod run;
pub mod window;

pub struct App {
    pub window: window::Window,
    pub clock: Clock,
    pub state: State,
}
