use core::{clock::Clock, event::Events};
use log::info;
use render::{GpuGlobals, Renderer};
use vek::Vec2;

use winit::{
    event_loop::{ControlFlow, EventLoop},
    keyboard::PhysicalKey,
};

use crate::{
    client::Client,
    input::KeyboardInput,
    window::{Window, WindowEvent},
};

pub fn run(event_loop: EventLoop<()>, mut client: Client) {
    info!("Running explora");
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut clock = Clock::default();

    event_loop
        .run(move |event, elwt| match event {
            winit::event::Event::AboutToWait => {
                let window = client.state_mut().resource_mut::<Window>();
                window.platform().request_redraw();
            },
            winit::event::Event::WindowEvent { event, window_id } => {
                let window = client.state_mut().resource_mut::<Window>();
                if window.platform().id() == window_id {
                    match event {
                        winit::event::WindowEvent::RedrawRequested => {
                            clock.tick();
                            let globals = *client.state_mut().resource::<GpuGlobals>();
                            let renderer = client.state_mut().resource_mut::<Renderer>();
                            renderer.write_globals(globals);
                            client.tick(clock.dt());
                        },
                        winit::event::WindowEvent::CloseRequested => elwt.exit(),
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
                        winit::event::WindowEvent::KeyboardInput { event, .. } => {
                            if let PhysicalKey::Code(code) = event.physical_key {
                                let keyboard_input =
                                    client.state_mut().resource_mut::<Events<KeyboardInput>>();
                                keyboard_input.send(KeyboardInput {
                                    scan_code: 0,
                                    key_code: Some(code),
                                    state: event.state == winit::event::ElementState::Pressed,
                                });
                            }
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
            _ => (),
        })
        .expect("Failed to run event loop");
}
