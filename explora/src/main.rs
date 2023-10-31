use common::{
    clock::Clock,
    state::{State, SysResult},
};
use explora::{camera::Camera, event::Events, input::Input, window::Window, App};
use render::{ Renderer, TerrainBuffer};
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
    let mut state = setup_ecs(renderer).expect("Failed to setup ECS. This is because one or more systems failed to run due to missing resources.");

    let names = state.ecs_mut().get_sync_schedule_names();

    log::debug!("Order of systems:");
    for (i, system) in names.iter().enumerate() {
        log::debug!("{}: {:?}", i, system);
    }

    let clock = Clock::default();
    let app = App {
        window,
        clock,
        state,
    };
    explora::run::run(event_loop, app);
}

fn setup_ecs(renderer: Renderer) -> apecs::anyhow::Result<State> {
    let mut state = State::new()?;
    state
        .ecs_mut()
        .with_default_resource::<Input>()?
        .with_default_resource::<Events<WindowEvent>>()?
        .with_default_resource::<TerrainBuffer>()?
        .with_resource(renderer)?
        .with_system("setup_system", setup)?
        .with_system("window_event_system", explora::window::window_event_system)?
        .with_system("render_system", render::render_system)?
        .with_system("camera_system", explora::camera::camera_system)?
        .with_system(
            "terrain_system_setup",
            explora::terrain::terrain_system_setup,
        )?;

    Ok(state)
}

use apecs::{end, Entities, NoDefault, Write};

fn setup((mut entities, _): (Write<Entities>, Write<Renderer, NoDefault>)) -> SysResult {
    let mut player = entities.create();
    // TODO: grab window / render surface size
    player.insert_component(Camera::new(800.0 / 600.0));
    end()
}
