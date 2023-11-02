pub mod scene;

use common::{
    clock::Clock,
    state::{State, SysResult},
};
use explora::{
    block::{self, BlockMap},
    camera::Camera,
    event::{Events, WindowEvent},
    input::{self, Input},
    window::Window,
    App,
};
use render::{GpuGlobals, Renderer, TerrainRenderData};

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
    let block_map = block::load_blocks("assets/blocks", &renderer.block_atlas().tiles);
    let mut state = setup_ecs(renderer, block_map).expect("Failed to setup ECS. This is because one or more systems failed to run due to missing resources.");

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

fn setup_ecs(renderer: Renderer, blocks: BlockMap) -> apecs::anyhow::Result<State> {
    let mut state = State::new()?;
    state
        .ecs_mut()
        .with_default_resource::<Input>()?
        .with_default_resource::<Events<WindowEvent>>()?
        .with_default_resource::<TerrainRenderData>()?
        .with_default_resource::<GpuGlobals>()?
        .with_resource(renderer)?
        .with_resource(blocks)?
        .with_system("setup", setup)?
        .with_system("terrain_setup", explora::terrain::terrain_system_setup)?
        .with_system_barrier()
        .with_system("game_input", input::game_input_system)?
        .with_system_barrier()
        .with_system("scene_update", scene_update_system)?
        .with_system_barrier()
        .with_system("render", render::render_system)?;

    Ok(state)
}

use apecs::{end, Entities, NoDefault, Write};
use scene::scene_update_system;

fn setup((mut entities, _): (Write<Entities>, Write<Renderer, NoDefault>)) -> SysResult {
    let mut player = entities.create();
    // TODO: grab window / render surface size
    player.insert_component(Camera::new(1920.0 / 1080.0));
    end()
}
