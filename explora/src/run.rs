use common::{clock::Clock, event::Events};
use log::info;
use vek::Vec2;

use winit::{
    event_loop::{ControlFlow, EventLoop},
    keyboard::PhysicalKey,
};

use crate::{
    client::Client,
    input::Input,
    render::{resources::EguiContext, Renderer},
    settings::GameplaySettings,
    ui::{EguiInput, EguiState},
    window::{Window, WindowEvent},
};

pub fn run(event_loop: EventLoop<()>, mut client: Client) {
    info!("Running explora");
    event_loop.set_control_flow(ControlFlow::Poll);
    let window = client.state().resource::<Window>().platform();
    let egui_context = client.state().resource::<EguiContext>();
    let mut egui_state = EguiState::new(egui_context.get(), window);
    event_loop
        .run(move |event, elwt| {
            match event {
                winit::event::Event::AboutToWait => {
                    let window = client.state_mut().resource_mut::<Window>();
                    window.platform().request_redraw();
                },
                winit::event::Event::WindowEvent { event, window_id } => {
                    let window = client.state_mut().resource_mut::<Window>();
                    let response = egui_state.state.on_window_event(window.platform(), &event);
                    if response.consumed {
                        // If the input was consumed by egui, we don't want to process it.
                        return;
                    }
                    if window.platform().id() == window_id {
                        match event {
                            winit::event::WindowEvent::CloseRequested => elwt.exit(),
                            winit::event::WindowEvent::Resized(size) => {
                                let renderer = client.state_mut().resource_mut::<Renderer>();
                                renderer.resize(size.width, size.height);

                                let events =
                                    client.state_mut().resource_mut::<Events<WindowEvent>>();
                                let new_size = Vec2::new(size.width, size.height);
                                events.send(WindowEvent::Resize(new_size));
                            },

                            winit::event::WindowEvent::ScaleFactorChanged { .. } => {
                                let size = window.platform().inner_size();
                                let events =
                                    client.state_mut().resource_mut::<Events<WindowEvent>>();
                                events
                                    .send(WindowEvent::Resize(Vec2::new(size.width, size.height)));
                            },

                            winit::event::WindowEvent::KeyboardInput { event, .. } => {
                                if let PhysicalKey::Code(code) = event.physical_key {
                                    let input = client.state_mut().resource_mut::<Input>();
                                    match event.state {
                                        winit::event::ElementState::Pressed => {
                                            input.press(code);
                                        },
                                        winit::event::ElementState::Released => {
                                            input.release(code);
                                        },
                                    }
                                }
                            },
                            winit::event::WindowEvent::RedrawRequested => {
                                let clock = client.state_mut().resource_mut::<Clock>();
                                clock.tick();

                                let window = client.state_mut().resource::<Window>();
                                let raw_input = egui_state.state.take_egui_input(window.platform());
                                client
                                    .state_mut()
                                    .resource_mut::<EguiInput>()
                                    .set(raw_input);

                                let clock = client.state().resource::<Clock>();
                                client.tick(clock.dt());
                            },
                            _ => (),
                        }
                    }
                },

                winit::event::Event::DeviceEvent {
                    event: winit::event::DeviceEvent::MouseMotion { delta: (dx, dy) },
                    ..
                } => {
                    let settings = client.state().resource::<GameplaySettings>();
                    let delta = Vec2::new(
                        dx as f32 * (settings.mouse_sensitivity as f32 / 100.0),
                        dy as f32 * (settings.mouse_sensitivity as f32 / 100.0),
                    );
                    let events = client.state_mut().resource_mut::<Events<WindowEvent>>();
                    events.send(WindowEvent::CursorMove(delta));
                },
                _ => (),
            }
        })
        .unwrap();
}
