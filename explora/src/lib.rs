use common::clock::Clock;

pub mod block;
pub mod camera;
pub mod client;
pub mod error;
pub mod input;
pub mod mesh;
pub mod run;
pub mod terrain;
pub mod window;

pub struct App {
    pub window: window::Window,
    pub clock: Clock,
}
