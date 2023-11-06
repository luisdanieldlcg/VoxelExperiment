pub mod scene;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use core::SysResult;
use explora::{
    block,
    camera::Camera,
    client::Client,
    input::{self, Input, KeyboardInput},
    window::{Window, WindowEvent},
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
    let mut client = Client::new(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 1234));
    setup_ecs(&mut client, window).expect("Failed to setup game state");
    explora::run::run(event_loop, client);
}

fn setup_ecs(client: &mut Client, window: Window) -> apecs::anyhow::Result<()> {
    let renderer = Renderer::new(window.platform()).expect("Failed to create renderer");
    let block_map = block::load_blocks("assets/blocks", &renderer.block_atlas().tiles);

    client
        .state_mut()
        .ecs_mut()
        .with_default_resource::<Input>()?
        .with_default_resource::<TerrainRenderData>()?
        .with_default_resource::<GpuGlobals>()?
        .with_resource(window)?
        .with_resource(renderer)?
        .with_resource(block_map)?
        .with_system("setup", setup)?
        .with_system_barrier()
        .with_system("terrain_setup", explora::terrain::terrain_system_setup)?
        .with_system_barrier()
        .with_system("keyboard_input_process", input::keyboard_input_system)?
        .with_system_barrier()
        .with_system("game_input", input::game_input_system)?
        .with_system_barrier()
        .with_system("scene_update", scene_update_system)?
        .with_system_barrier()
        .with_system("render", render::render_system)?;

    client
        .state_mut()
        .with_event::<WindowEvent>("window_event")
        .with_event::<KeyboardInput>("keyboard_input_event");

    let names = client.state_mut().ecs_mut().get_sync_schedule_names();
    log::debug!("System schedule order:");
    for (i, system) in names.iter().enumerate() {
        log::debug!("{}: {:?}", i, system);
    }
    Ok(())
}

use apecs::*;
use scene::scene_update_system;

#[derive(CanFetch)]
struct SetupSystem {
    entities: Write<Entities>,
    window: Write<Window, NoDefault>,
}

fn setup(mut sys: SetupSystem) -> SysResult {
    sys.window.grab_cursor(true);
    let mut player = sys.entities.create();
    let window_size = sys.window.inner_size().map(|x| x as f32);
    let aspect_ratio = window_size.x / window_size.y;
    let mut camera = Camera::new(aspect_ratio);
    camera.rotate(0.0, 0.0);
    player.insert_component(camera);
    end()
}
