use common::{clock::Clock, resources::GameMode};
use explora::client;
use explora::render::Renderer;
use explora::{
    block::BlockMap,
    client::Client,
    input::{self, Input},
    scene,
    singleplayer::Singleplayer,
    ui::EguiInput,
    window::{Window, WindowEvent},
};
fn main() -> apecs::anyhow::Result<()> {
    common::init_logger("wgpu=warn,naga=error,apecs=warn");

    let (window, event_loop) = Window::new().unwrap_or_else(|error| match error {
        explora::error::Error::Window(e) => panic!("{:?}", e),
    });
    let singleplayer = Singleplayer::init();
    let addr = singleplayer.wait_for_init();
    let aspect = window.inner_size().x as f32 / window.inner_size().y as f32;
    let mut client = match Client::new(addr, aspect) {
        Ok(t) => t,
        Err(err) => {
            log::error!("{:?}", err);
            // TODO: if we cannot connect to the server create a single-player game
            panic!();
        },
    };
    initialize_ecs(&mut client, window)?;
    // TODO: change this. this should NOT be here
    *client.state_mut().resource_mut::<GameMode>() = GameMode::Singleplayer;
    explora::run::run(event_loop, client);
    Ok(())
}

fn initialize_ecs(client: &mut Client, window: Window) -> apecs::anyhow::Result<()> {
    let block_map = BlockMap::load_blocks("assets/blocks", "assets/textures/blocks");
    let render_plugin = Renderer::initialize(window.platform(), block_map.textures()).unwrap();

    client
        .state_mut()
        .ecs_mut()
        .with_resource(block_map)?
        .with_default_resource::<Clock>()?
        .with_default_resource::<Input>()?
        .with_default_resource::<EguiInput>()?
        .with_resource(window)?
        .with_plugin(render_plugin)?
        .with_system_with_dependencies(
            explora::terrain::TERRAIN_CHUNK_MESH_SYSTEM,
            explora::terrain::terrain_chunk_mesh,
            &[client::CHUNK_LOAD_SYSTEM],
            &[],
        )?
        .with_system_with_dependencies(
            explora::render::SYSTEM_STAGE_UI_DRAW_WIDGETS,
            explora::ui::ui_debug_render_system,
            &[],
            &[],
        )?
        .with_system_barrier()
        .with_system("scene_update", scene::scene_update_system)?
        .with_system_barrier()
        .with_system("input", input::input_system)?;

    client.state_mut().with_event::<WindowEvent>("window_event");
    common::state::print_system_schedule(client.state_mut().ecs_mut());
    Ok(())
}
