use core::{clock::Clock, event::Events};
use log::info;
use render::{EguiContext, Renderer};
use vek::Vec2;

use winit::event_loop::EventLoop;

use crate::{
    client::Client,
    input::KeyboardInput,
    ui::{EguiInput, EguiState},
    window::{Window, WindowEvent},
};

pub fn run(event_loop: EventLoop<()>, mut client: Client) {
    info!("Running explora");
    // event_loop.set_control_flow(ControlFlow::Poll);
    let mut egui_state = EguiState::new(client.state().resource::<Window>().platform());

    event_loop.run(move |event, _, control| {
        control.set_poll();
        match event {
            winit::event::Event::MainEventsCleared => {
                let window = client.state_mut().resource_mut::<Window>();
                window.platform().request_redraw();
            },
            winit::event::Event::WindowEvent { event, window_id } => {
                let egui_context = client.state().resource::<EguiContext>();

                let input = egui_state.state.on_event(egui_context.get(), &event);
                if input.consumed {
                    // TODO: check egui input
                }
                let window = client.state_mut().resource_mut::<Window>();

                if window.platform().id() == window_id {
                    match event {
                        winit::event::WindowEvent::CloseRequested => control.set_exit(),
                        winit::event::WindowEvent::Resized(size) => {
                            let renderer = client.state_mut().resource_mut::<Renderer>();
                            renderer.resize(size.width, size.height);

                            let events = client.state_mut().resource_mut::<Events<WindowEvent>>();
                            let new_size = Vec2::new(size.width, size.height);
                            events.send(WindowEvent::Resize(new_size));
                        },
                        winit::event::WindowEvent::ScaleFactorChanged { .. } => {
                            let size = window.platform().inner_size();
                            let events = client.state_mut().resource_mut::<Events<WindowEvent>>();
                            events.send(WindowEvent::Resize(Vec2::new(size.width, size.height)));
                        },
                        winit::event::WindowEvent::KeyboardInput { input, .. } => {
                            if let Some(key) = input.virtual_keycode {
                                let keyboard_input =
                                    client.state_mut().resource_mut::<Events<KeyboardInput>>();
                                keyboard_input.send(KeyboardInput {
                                    scan_code: 0,
                                    key_code: Some(key),
                                    state: input.state == winit::event::ElementState::Pressed,
                                });
                            }
                            // if let PhysicalKey::Code(code) = event.physical_key {
                            //     let keyboard_input =
                            //         client.state_mut().resource_mut::<Events<KeyboardInput>>();
                            //     keyboard_input.send(KeyboardInput {
                            //         scan_code: 0,
                            //         key_code: Some(code),
                            //         state: event.state == winit::event::ElementState::Pressed,
                            //     });
                            // }
                        },
                        _ => (),
                    }
                }
            },

            winit::event::Event::DeviceEvent {
                event: winit::event::DeviceEvent::MouseMotion { delta },
                ..
            } => {
                let events = client.state_mut().resource_mut::<Events<WindowEvent>>();
                let delta = Vec2::new(delta.0 as f32, delta.1 as f32);
                events.send(WindowEvent::CursorMove(delta));
            },

            winit::event::Event::RedrawRequested(_) => {
                let clock = client.state_mut().resource_mut::<Clock>();
                clock.tick();

                // Take winit input before beginning egui frame
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
    });
}
