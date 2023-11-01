use crate::error::Error;

use winit::event_loop::EventLoop;

pub struct Window {
    platform: winit::window::Window,
}

impl Window {
    pub fn new() -> Result<(Self, EventLoop<()>), Error> {
        let event_loop = winit::event_loop::EventLoop::new().expect("Failed to create event loop");
        let platform = winit::window::WindowBuilder::new()
            .with_title("Explora")
            .with_inner_size(winit::dpi::PhysicalSize::new(800, 600))
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
