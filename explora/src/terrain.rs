use crate::event::{Events, GameInputEvent};
use common::{
    chunk::Chunk,
    ecs::{end, ok, NoDefault, Read, Write},
    resources::TerrainMap,
    state::SysResult,
};
use render::{Renderer, TerrainData};
use vek::Vec2;

use apecs::*;

#[derive(CanFetch)]
pub struct TerrainSetupSystem {
    renderer: Write<Renderer, NoDefault>,
    terrain: Write<TerrainMap>,
    terrain_buffer: Write<TerrainData, NoDefault>,
}
pub fn terrain_setup_system(mut sys: TerrainSetupSystem) -> SysResult {
    for x in -2..1 {
        for z in -2..1 {
            let chunk_pos = Vec2::new(x, z);
            let chunk = Chunk::generate(chunk_pos);
            sys.terrain.0.insert(chunk_pos, chunk);
        }
    }
    let mut mesh_work = vec![];
    for (pos, chunk) in sys.terrain.0.iter() {
        let mesh = render::mesh::create_chunk_mesh(chunk, *pos, sys.terrain.inner());
        mesh_work.extend(mesh);
    }

    *sys.terrain_buffer = TerrainData {
        buffer: Some(sys.renderer.create_vertex_buffer(&mesh_work)),
        show_wireframe: false,
    };
    end()
}

#[derive(CanFetch)]
pub struct TerrainTickSystem {
    terrain_buffer: Write<TerrainData, NoDefault>,
    game_events: Read<Events<GameInputEvent>, NoDefault>,
}

pub fn terrain_tick_system(mut terrain: TerrainTickSystem) -> SysResult {
    for event in &terrain.game_events.events {
        if let GameInputEvent::ToggleWireframe = event {
            terrain.terrain_buffer.show_wireframe = !terrain.terrain_buffer.show_wireframe;
        }
    }
    ok()
}
