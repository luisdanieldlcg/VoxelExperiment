use crate::{error::Error, input::GameInput};

use vek::Vec2;
use winit::event_loop::EventLoop;

/// Represents the various window events that are relevant for the game.
#[derive(Debug, Clone, Copy)]
pub enum WindowEvent {
    /// The window has been requested to close.
    Close,
    /// The window has been resized.
    Resize(Vec2<u32>),
    /// The cursor has been moved.
    CursorMove(Vec2<f32>),
    /// A game key has been pressed.
    KeyPress(GameInput, bool),
}

pub struct Window {
    platform: winit::window::Window,
}

impl Window {
    pub fn new() -> Result<(Self, EventLoop<()>), Error> {
        let event_loop = winit::event_loop::EventLoop::new().expect("Failed to create event loop");
        let platform = winit::window::WindowBuilder::new()
            .with_title("Explora")
            .with_inner_size(winit::dpi::PhysicalSize::new(1920, 1080))
            .build(&event_loop)?;

        let this = Self { platform };
        Ok((this, event_loop))
    }
    pub fn trap_cursor(&self, value: bool) {
        self.platform.set_cursor_visible(!value);
        let mode = if value {
            winit::window::CursorGrabMode::Locked
        } else {
            winit::window::CursorGrabMode::None
        };
        if let Err(e) = self.platform.set_cursor_grab(mode) {
            log::warn!("Could not grab cursor in {:?} mode ({})", mode, e);
        }
    }

    pub fn platform(&self) -> &winit::window::Window {
        &self.platform
    }
}
