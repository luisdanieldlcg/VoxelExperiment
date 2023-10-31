use common::{chunk::Chunk, resources::TerrainMap, state::SysResult};
use render::{Renderer, TerrainBuffer};
use vek::Vec2;

use apecs::*;

#[derive(CanFetch)]
pub struct TerrainSystem {
    renderer: Write<Renderer, NoDefault>,
    terrain: Write<TerrainMap>,
    terrain_buffer: Write<TerrainBuffer, NoDefault>,
}

pub fn terrain_system_setup(mut system: TerrainSystem) -> SysResult {
    let chunk = Chunk::generate(Vec2::zero());
    let mesh = render::mesh::create_chunk_mesh(&chunk);
    system.terrain.0.insert(Vec2::zero(), chunk);
    *system.terrain_buffer = TerrainBuffer(Some(system.renderer.create_vertex_buffer(&mesh)));
    end()
}
