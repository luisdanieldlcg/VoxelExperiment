use common::{clock::Clock, state::State};

pub mod block;
pub mod camera;
pub mod error;
pub mod event;
pub mod input;
pub mod mesh;
pub mod run;
pub mod terrain;
pub mod window;

pub struct App {
    pub window: window::Window,
    pub clock: Clock,
    pub state: State,
}
