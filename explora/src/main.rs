use common::{
    clock::Clock,
    ecs::{Entities, NoDefault, ShouldContinue, Write},
    state::{State, SysResult},
};
use explora::{
    camera::{camera_update_system, Camera},
    event::{Events, GameInputEvent, WindowEvent},
    terrain::terrain_setup_system,
    window::{window_event_system, Window},
    App,
};
use log::debug;
use render::{Renderer, TerrainData};

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

    let clock = Clock::default();
    let mut state = setup_ecs(renderer).expect(
        "Failed to setup ECS.  One or more systems failed to fetch necessary resources. This is a bug",
    );

    let names = state.ecs_mut().get_sync_schedule_names();

    debug!("Order of systems:");
    for (i, system) in names.iter().enumerate() {
        debug!("{}: {:?}", i, system);
    }

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
        .with_default_resource::<Events<WindowEvent>>()?
        .with_default_resource::<Events<GameInputEvent>>()?
        .with_default_resource::<TerrainData>()?
        .with_resource(renderer)?
        .with_system("setup", setup_system)?
        .with_system_barrier()
        .with_system_with_dependencies("window_event", window_event_system, &[], &["terrain_setup"])?
        .with_system_with_dependencies(
            "camera_update",
            camera_update_system,
            &["window_event"],
            &[],
        )?
        .with_system_barrier()
        .with_system("terrain_setup", terrain_setup_system)?
        .with_system("terrain_tick", explora::terrain::terrain_tick_system)?
        .with_system("render", render::render_system)?;

    Ok(state)
}

fn setup_system((mut entities, _): (Write<Entities>, Write<Renderer, NoDefault>)) -> SysResult {
    let mut player = entities.create();
    // TODO: grab window / render surface size
    player.insert_component(Camera::new(800.0 / 600.0));
    Ok(ShouldContinue::No)
}
