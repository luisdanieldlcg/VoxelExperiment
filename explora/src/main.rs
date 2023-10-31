use common::{
    clock::Clock,
    ecs::{Entities, NoDefault, ShouldContinue, Write},
    state::{State, SysResult},
};
use explora::{camera::Camera, event::Events, input::Input, window::Window, App};
use render::{Renderer, buffer::Buffer, vertex::TerrainVertex, TerrainBuffer};
use winit::event::WindowEvent;

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

    window.trap_cursor(true);

    let mut state = State::new();
    state.add_resource(Input::default());
    state.add_resource(Events::<WindowEvent>::default());
    state.add_resource(renderer);
    state.add_resource(TerrainBuffer(None));
    state.add_system("setup_system", setup);
    state.add_system("window_event_system", explora::window::window_event_system);
    state.add_system("render_system", render::render_system);
    state.add_system("camera_system", explora::camera::camera_system);
    state.add_system("terrain_system_setup", explora::terrain::terrain_system_setup);

    let clock = Clock::default();
    let app = App {
        window,
        clock,
        state,
    };
    explora::run::run(event_loop, app);
}

fn setup((mut entities, _): (Write<Entities>, Write<Renderer, NoDefault>)) -> SysResult {
    let mut player = entities.create();
    // TODO: grab window / render surface size
    player.insert_component(Camera::new(800.0 / 600.0));
    Ok(ShouldContinue::No)
}
