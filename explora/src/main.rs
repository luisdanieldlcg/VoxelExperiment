use common::{clock::Clock, state::State};
use explora::{input::Input, window::Window, App};
use render::Renderer;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .filter_module("wgpu_core", log::LevelFilter::Info)
        .init();

    let (window, event_loop) = Window::new().unwrap_or_else(|error| match error {
        explora::error::Error::Window(e) => panic!("{:?}", e),
    });
    let Ok(renderer) = Renderer::new(window.platform()) else {
        // TODO: proper error handling
        panic!("Failed to create renderer");
    };

    let mut state = State::new();
    state.add_resource(Input::default());
    state.add_resource(renderer);
    state.add_system("render_system", render::render_system);

    let clock = Clock::default();

    let app = App {
        window,
        clock,
        state,
    };

    explora::run::run(event_loop, app);
}
