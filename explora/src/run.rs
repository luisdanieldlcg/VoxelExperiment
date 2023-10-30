use crate::event::{GameInputEvent, WindowEvent};
use crate::{event::Events, input::Input, App};
use apecs::*;
use apecs::{ok, Write};
use log::{info, debug};
use vek::Vec2;
use winit::{
    event::{DeviceEvent, ElementState, Event},
    event_loop::{ControlFlow, EventLoop},
    keyboard::PhysicalKey,
};

pub fn run(event_loop: EventLoop<()>, mut app: App) {
    info!("Running explora");
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run(move |event, elwt| {
            let  window_events = app.state.resource_mut::<Events<WindowEvent>>();
            match event {
                Event::AboutToWait => {
                    app.window.platform().request_redraw();
                },
                Event::WindowEvent { event, window_id }
                    if window_id == app.window.platform().id() =>
                {
                    match event {
                        winit::event::WindowEvent::CloseRequested => elwt.exit(),
                        winit::event::WindowEvent::Resized(size) => {
                            let new_size = Vec2::new(size.width, size.height);
                            window_events.push(WindowEvent::Resize(new_size));
                        },
                        winit::event::WindowEvent::ScaleFactorChanged { .. } => {
                            let size = app.window.platform().inner_size();

                            window_events
                                .push(WindowEvent::Resize(Vec2::new(size.width, size.height)));
                        },
                        winit::event::WindowEvent::KeyboardInput { event, .. } => {
                            if let PhysicalKey::Code(code) = event.physical_key {
                                window_events.push(WindowEvent::KeyPress(
                                    code,
                                    event.state == ElementState::Pressed,
                                ));
                            }
                        },

                        winit::event::WindowEvent::MouseInput { state, button, .. } => {
                            window_events.push(WindowEvent::ButtonPress(
                                button,
                                state == ElementState::Pressed,
                            ));
                        },
                        winit::event::WindowEvent::RedrawRequested => {
                            // TODO: try to move state update to here
                        }
                        _ => (),
                    }
                },
                winit::event::Event::DeviceEvent {
                    event: DeviceEvent::MouseMotion { delta },
                    ..
                } => {
                    let delta = Vec2::new(delta.0 as f32, delta.1 as f32);

                    window_events.push(WindowEvent::CursorMove(delta));
                },
                _ => (),
            }

            app.state.tick(app.clock.dt());
            app.clock.tick();

            let input = app.state.resource::<Input>();

            if input.is_key_down(winit::keyboard::KeyCode::Escape) {
                elwt.exit();
            }
            let window_events = app.state.resource_mut::<Events<WindowEvent>>();
            window_events.clear();
        })
        .unwrap();
}
