use common::{chunk::Chunk, resources::TerrainMap, SysResult};

use crate::render::{resources::TerrainRender, ChunkPos, Renderer};

use apecs::*;
use vek::Vec2;

use crate::{block::BlockMap, mesh};

#[derive(CanFetch)]
pub struct TerrainSystem {
    renderer: Write<Renderer, NoDefault>,
    terrain_map: Write<TerrainMap>,
    block_map: Read<BlockMap>,
    terrain_render_data: Write<TerrainRender, NoDefault>,
}

pub fn terrain_system_render(mut system: TerrainSystem) -> SysResult {
    let blocks = system.block_map.inner();

    let terrain = system.terrain_map.inner();

    for (pos, chunk) in terrain.chunks.iter() {
        let neighbors = [
            terrain.chunks.get(&(pos + Vec2::new(0, 1))),
            terrain.chunks.get(&(pos + Vec2::new(1, 0))),
            terrain.chunks.get(&(pos + Vec2::new(0, -1))),
            terrain.chunks.get(&(pos + Vec2::new(-1, 0))),
        ];

        if neighbors.iter().any(|n| n.is_none()) {
            continue;
        }

        if system.terrain_render_data.chunks.get(pos).is_none() {
            // create the mesh of the chunk
            let vertices = mesh::create_chunk_mesh(chunk, *pos, &system.terrain_map, blocks);
            let buffer = system.renderer.create_vertex_buffer(&vertices);
            let chunk_pos = ChunkPos::new(pos.x, pos.y);
            let terrain_mesh = system.renderer.create_terrain_chunk_mesh(chunk_pos, buffer);
            system.terrain_render_data.chunks.insert(*pos, terrain_mesh);
        }
    }

    ok()
}
