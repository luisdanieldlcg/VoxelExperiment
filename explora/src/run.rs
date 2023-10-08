use crate::{App, input::Input};
use log::{info, debug};
use winit::{
    event::{Event, WindowEvent, ElementState},
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
                    WindowEvent::KeyboardInput {  input, .. } => {
                        let res = app.state.resource_mut::<Input>();
                            debug!("pressed key: {:?}", input);

                        res.buttons[input.scancode as usize] = input.state == ElementState::Pressed;
                        if let Some(key) = input.virtual_keycode {
                            res.keys[key as usize] = input.state == ElementState::Pressed;
                        }
                    }
                    _ => (),
                }
            },
            _ => (),
        }
        app.state.tick(app.clock.dt());
        app.clock.tick();
        // let delta = app.state.resource::<DeltaTime>().0;
        // log::debug!("Delta Time: {}", delta);
        // let _ = app.state.resource::<Input>();

    });
}
