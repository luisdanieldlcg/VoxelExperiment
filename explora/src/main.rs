use core::{clock::Clock, SysResult, resources::GameMode};
use explora::{
    block::{self, BlockMap},
    camera::Camera,
    client::Client,
    input::{self, Input, KeyboardInput},
    scene,
    singleplayer::Singleplayer,
    terrain::terrain_system_setup,
    ui::EguiInput,
    window::{Window, WindowEvent},
};
use render::Renderer;

fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .filter_module("wgpu", log::LevelFilter::Warn)
        .filter_module("naga", log::LevelFilter::Info)
        .init();

    let (window, event_loop) = Window::new().unwrap_or_else(|error| match error {
        explora::error::Error::Window(e) => panic!("{:?}", e),
    });

    let singleplayer = Singleplayer::init();
    let addr = singleplayer.wait_for_init();
    let mut client = match Client::new(addr) {
        Ok(t) => t,
        Err(err) => {
            log::error!("{:?}", err);
            // TODO: if we cannot connect to the server create a single-player game
            panic!();
        },
    };
    setup_ecs(&mut client, window)?;
    // TODO: change this. this should NOT be here
    *client.state_mut().resource_mut::<GameMode>() = GameMode::Singleplayer;
    explora::run::run(event_loop, client);
    Ok(())
}

fn setup_ecs(client: &mut Client, window: Window) -> anyhow::Result<()> {
    let render_plugin = Renderer::initialize(window.platform()).unwrap();
    client
        .state_mut()
        .ecs_mut()
        .with_default_resource::<Clock>()?
        .with_default_resource::<Input>()?
        .with_default_resource::<EguiInput>()?
        .with_default_resource::<BlockMap>()?
        .with_resource(window)?
        .with_system_barrier()
        .with_plugin(render_plugin)?
        .with_system_with_dependencies(
            render::SYSTEM_STAGE_UI_DRAW_WIDGETS,
            explora::ui::ui_debug_render_system,
            &[],
            &[],
        )?
        .with_system_with_dependencies("setup", setup, &[], &[render::SYSTEM_STAGE_PRE_RENDER])?
        .with_system_with_dependencies(
            "terrain_setup",
            terrain_system_setup,
            &[render::SYSTEM_STAGE_PRE_RENDER],
            &[],
        )?
        .with_system("keyboard_input_process", input::keyboard_input_system)?
        .with_system_barrier()
        .with_system("game_input", input::game_input_system)?
        .with_system_barrier()
        .with_system("scene_update", scene::scene_update_system)?
        .with_system_barrier();

    client
        .state_mut()
        .with_event::<WindowEvent>("window_event")
        .with_event::<KeyboardInput>("keyboard_input_event");

    core::state::print_system_schedule(client.state_mut().ecs_mut());

    Ok(())
}

use apecs::*;

#[derive(CanFetch)]
struct SetupSystem {
    entities: Write<Entities>,
    window: Write<Window, NoDefault>,
    block_map: Write<BlockMap>,
    renderer: Read<Renderer, NoDefault>,
}

fn setup(mut sys: SetupSystem) -> SysResult {
    sys.window.grab_cursor(true);
    *sys.block_map = block::load_blocks("assets/blocks", &sys.renderer.block_atlas().tiles);
    let mut player = sys.entities.create();
    let window_size = sys.window.inner_size().map(|x| x as f32);
    let aspect_ratio = window_size.x / window_size.y;
    let mut camera = Camera::new(aspect_ratio);
    camera.rotate(0.0, 0.0);
    player.insert_component(camera);
    end()
}
