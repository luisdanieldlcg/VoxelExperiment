use std::{sync::Arc, time::Instant};

use vek::{Vec2, Vec3};
use winit::{
    event::{DeviceEvent, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{renderer::{Renderer, Uniforms}, scene::Scene};

pub struct Window {
    platform: Arc<winit::window::Window>,
    event_loop: Option<EventLoop<()>>,
    renderer: Renderer,
}

impl Window {
    pub fn new(width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new().expect("EventLoop is ok");
        let window = WindowBuilder::new()
            .with_title("explora")
            .with_inner_size(winit::dpi::PhysicalSize::new(width, height))
            .build(&event_loop)
            .unwrap();
        let window = Arc::new(window);
        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        event_loop.set_control_flow(ControlFlow::Poll);
        let renderer = Renderer::new(window.clone());
        Self {
            platform: window,
            event_loop: Some(event_loop),
            renderer,
        }
    }
    pub fn run(&mut self) {
        let mut last_frame = Instant::now();
        let mut key_state = KeyState::default();
        let size = self.platform.inner_size();
        let mut scene = Scene::new(size.width as f32 / size.height as f32);
        let _ = self.event_loop.take().unwrap().run(move |event, elwt| {
            match event {
                Event::WindowEvent { window_id, event } if window_id == self.platform.id() => {
                    match event {
                        WindowEvent::CloseRequested => {
                            tracing::info!("Application close requested.");
                            elwt.exit();
                        },
                        WindowEvent::KeyboardInput { event, .. } => {
                            if let winit::keyboard::PhysicalKey::Code(code) = event.physical_key {
                                key_state.update(
                                    code,
                                    event.state == winit::event::ElementState::Pressed,
                                );
                            }
                        },
                        WindowEvent::Resized(size) => {
                            self.renderer.resize(size.width, size.height);
                            scene.resize(size.width as f32 / size.height as f32);
                        },
                        WindowEvent::ScaleFactorChanged { .. } => {
                            let size = self.platform.inner_size();
                            self.renderer.resize(size.width, size.height);
                            scene.resize(size.width as f32 / size.height as f32);
                        },
                        _ => (),
                    }
                },
                Event::AboutToWait => {
                    // Application update code.

                    // Queue a RedrawRequested event.
                    //
                    // You only need to call this if you've determined that you need to redraw in
                    // applications which do not always need to. Applications that redraw continuously
                    // can render here instead.
                    scene.set_movement_dir(Vec3::new(
                        key_state.right as i32 as f32 - key_state.left as i32 as f32,
                        key_state.up as i32 as f32 - key_state.down as i32 as f32,
                        key_state.forward as i32 as f32 - key_state.backward as i32 as f32,
                    ));

                    let now = Instant::now();
                    let dt = now - last_frame;
                    let matrices = scene.update(dt.as_secs_f32());
                    self.renderer.write_uniforms(Uniforms::new(matrices.view, matrices.proj));
                    self.renderer.render();
                    last_frame = now;
                },
                Event::DeviceEvent {event: DeviceEvent::MouseMotion { delta: (dx, dy) }, .. } => {
                    let sensitivity = 100.0;
                    let delta = Vec2::new(
                        dx as f32 * (sensitivity / 100.0),
                        dy as f32 * (sensitivity / 100.0),
                    );
                    scene.look(delta.x, delta.y);
                }
                _ => (),
            }
        });
    }

    pub fn grab_cursor(&mut self, value: bool) {
        self.platform.set_cursor_visible(!value);
        let mode = if value {
            winit::window::CursorGrabMode::Locked
        } else {
            winit::window::CursorGrabMode::None
        };
        if let Err(e) = self.platform.set_cursor_grab(mode) {
            tracing::warn!("Could not grab cursor in {:?} mode ({})", mode, e);
        }
        // self.cursor_grabbed = value;
    }
}

#[derive(Default, Debug)]
struct KeyState {
    forward: bool,
    backward: bool,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

impl KeyState {
    fn update(&mut self, key: winit::keyboard::KeyCode, pressed: bool) {
        match key {
            winit::keyboard::KeyCode::KeyW => self.forward = pressed,
            winit::keyboard::KeyCode::KeyS => self.backward = pressed,
            winit::keyboard::KeyCode::KeyA => self.left = pressed,
            winit::keyboard::KeyCode::KeyD => self.right = pressed,
            winit::keyboard::KeyCode::Space => self.up = pressed,
            winit::keyboard::KeyCode::ShiftLeft => self.down = pressed,
            _ => (),
        }
    }
}
