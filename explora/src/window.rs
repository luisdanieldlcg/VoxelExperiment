use crate::{
    camera::Camera,
    error::Error,
    event::{Events, GameInputEvent},
    input::Input,
};

use common::{
    ecs::{NoDefault, Query, Read, ShouldContinue, Write},
    resources::DeltaTime,
    state::SysResult,
};

use log::{debug, info};
use render::Renderer;
use winit::event_loop::EventLoop;

use crate::event::WindowEvent;

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

use apecs::*;

#[derive(CanFetch)]
pub struct WindowEventSystem<'a> {
    window_events: Write<Events<WindowEvent>>,
    input_events: Write<Events<GameInputEvent>>,
    input: Write<Input>,
    renderer: Write<Renderer, NoDefault>,
    camera: Query<&'a mut Camera>,
    delta_time: Read<DeltaTime>,
}

pub fn window_event_system(mut system: WindowEventSystem) -> SysResult {
    for event in &system.window_events.events {
        match event {
            WindowEvent::Resize(size) => {
                system.renderer.resize(size.x, size.y);
                for camera in system.camera.query().iter_mut() {
                    camera.set_aspect_ratio(size.x as f32 / size.y as f32);
                }
            },
            WindowEvent::KeyPress(key, pressed) => {
                if let Some(game_input_event) = Input::map_game_input(*key) {
                    system.input_events.push(game_input_event);
                }
                system.input.keys[*key as usize] = *pressed;
            },
            WindowEvent::ButtonPress(button, pressed) => {
                let code = match button {
                    winit::event::MouseButton::Left => 0,
                    winit::event::MouseButton::Right => 1,
                    winit::event::MouseButton::Middle => 2,
                    winit::event::MouseButton::Back => 3,
                    winit::event::MouseButton::Forward => 4,
                    winit::event::MouseButton::Other(code) => *code as usize,
                };
                system.input.buttons[code] = *pressed;
            },

            WindowEvent::CursorMove(delta) => {
                system.input.cursor_delta = *delta;
                for camera in system.camera.query().iter_mut() {
                    camera.rotate(delta.x, delta.y, system.delta_time.0);
                }
            },
            _ => {},
        }
    }
    ok()
}
