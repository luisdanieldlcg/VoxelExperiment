use common::{clock::Clock, state::State};
use explora::{input::Input, window::Window, App};

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let (window, event_loop) = Window::new().unwrap_or_else(|error| match error {
        explora::error::Error::Window(e) => panic!("{:?}", e),
    });

    let mut state = State::new();
    state.add_resource(Input::default());

    let clock = Clock::default();

    let app = App {
        window,
        clock,
        state,
    };
    
    explora::run::run(event_loop, app);
}
