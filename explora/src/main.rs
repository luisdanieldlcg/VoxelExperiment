use explora::window::Window;
use winit::{event::*, event_loop::*};

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .init();

    log::info!("Test info");
    log::debug!("Test debug");
    log::warn!("Test warn");
    log::error!("Test error");
    log::trace!("Test trace");

    let (window, event_loop) = Window::new().unwrap_or_else(|error| match error {
        explora::error::Error::Window(e) => panic!("{:?}", e),
    });

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, window_id } if window_id == window.platform().id() => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(..) => {},
                    WindowEvent::ScaleFactorChanged { .. } => {},
                    _ => (),
                }
            },
            _ => (),
        }
    });
}
