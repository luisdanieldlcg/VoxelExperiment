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
    ///
    /// This is true for every frame that the key is pressed.
    KeyPress(GameInput),
    /// A game key has just been pressed.
    ///
    /// This is only true for the first frame that the key is pressed.
    JustPressed(GameInput),
}

pub struct Window {
    platform: winit::window::Window,
    cursor_grabbed: bool,
}

impl Window {
    pub fn new() -> Result<(Self, EventLoop<()>), Error> {
        let event_loop = winit::event_loop::EventLoop::new();
        let platform = winit::window::WindowBuilder::new()
            .with_title("Explora")
            .with_inner_size(winit::dpi::PhysicalSize::new(1920, 1080))
            .build(&event_loop)?;

        let this = Self {
            platform,
            cursor_grabbed: true,
        };
        Ok((this, event_loop))
    }

    pub fn grab_cursor(&mut self, value: bool) {
        self.platform.set_cursor_visible(!value);
        let mode = if value {
            winit::window::CursorGrabMode::Locked
        } else {
            winit::window::CursorGrabMode::None
        };
        if let Err(e) = self.platform.set_cursor_grab(mode) {
            log::warn!("Could not grab cursor in {:?} mode ({})", mode, e);
        }
        self.cursor_grabbed = value;
    }

    pub fn inner_size(&self) -> Vec2<u32> {
        let size = self.platform.inner_size();
        Vec2::new(size.width, size.height)
    }

    pub fn toggle_cursor(&mut self) {
        self.grab_cursor(!self.cursor_grabbed);
    }

    pub fn platform(&self) -> &winit::window::Window {
        &self.platform
    }
}
