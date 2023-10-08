use crate::{input::Input, App};
use log::info;
use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

pub fn run(event_loop: EventLoop<()>, mut app: App) {
    info!("Running explora");
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent { event, window_id } if window_id == app.window.platform().id() => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(..) => {},
                    WindowEvent::ScaleFactorChanged { .. } => {},
                    WindowEvent::KeyboardInput { input, .. } => {
                        let res = app.state.resource_mut::<Input>();

                        res.buttons[input.scancode as usize] = input.state == ElementState::Pressed;
                        if let Some(key) = input.virtual_keycode {
                            res.keys[key as usize] = input.state == ElementState::Pressed;
                        }
                    },
                    _ => (),
                }
            },
            _ => (),
        }
        app.state.tick(app.clock.dt());
        app.clock.tick();

        let input = app.state.resource::<Input>();
        if input.is_key_down(winit::event::VirtualKeyCode::Escape) {
            // just for testing
            *control_flow = ControlFlow::Exit;
        }

    });
}
