use common::{
    chunk::Chunk,
    ecs::{end, ok, NoDefault, Write},
    resources::TerrainMap,
    state::SysResult,
};
use render::{Renderer, TerrainBuffer};
use vek::Vec2;

#[allow(clippy::type_complexity)]
pub fn terrain_system_setup(
    (mut renderer, mut terrain, mut terrain_buffer): (
        Write<Renderer, NoDefault>,
        Write<TerrainMap>,
        Write<TerrainBuffer, NoDefault>,
    ),
) -> SysResult {
    let chunk = Chunk::generate(Vec2::zero());
    let mesh = render::mesh::create_chunk_mesh(&chunk);
    terrain.0.insert(Vec2::zero(), chunk);
    *terrain_buffer = TerrainBuffer(Some(renderer.create_vertex_buffer(&mesh)));
    end()
}
