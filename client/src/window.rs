use std::sync::Arc;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::renderer::Renderer;

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
        let _ = self.event_loop.take().unwrap().run(move |event, elwt| {
            match event {
                Event::WindowEvent { window_id, event } if window_id == self.platform.id() => {
                    match event {
                        WindowEvent::CloseRequested => {
                            tracing::info!("Application close requested.");
                            elwt.exit();
                        },
                        WindowEvent::RedrawRequested => {
                            // Redraw the application.
                            //
                            // It's preferable for applications that do not render continuously to render in
                            // this event rather than in AboutToWait, since rendering in here allows
                            // the program to gracefully handle redraws requested by the OS.
                            self.renderer.render();
                        },
                        WindowEvent::Resized(size) => {
                            self.renderer.resize(size.width, size.height);
                        },
                        WindowEvent::ScaleFactorChanged { .. } => {
                            let size = self.platform.inner_size();
                            self.renderer.resize(size.width, size.height);
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
                    self.platform.request_redraw();
                },
                _ => (),
            }
        });
    }
}
