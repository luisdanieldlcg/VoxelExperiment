use core::{resources::TerrainMap, SysResult};
use render::{
    resources::{TerrainRender, TerrainRenderData},
    Renderer,
};

use apecs::*;

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

    let terrain = system.terrain_map.inner_mut();

    for (pos, chunk) in &terrain.chunks {
        if system.terrain_render_data.chunks.get(pos).is_none() {
            let mesh = mesh::create_chunk_mesh(chunk, *pos, terrain, blocks);
            system.terrain_render_data.chunks.insert(
                *pos,
                TerrainRenderData {
                    buffer: Some(system.renderer.create_vertex_buffer(&mesh)),
                },
            );
        }
    }
    ok()
}
