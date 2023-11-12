use core::{ resources::TerrainMap, SysResult};

use render::{
    resources::{TerrainRender, TerrainRenderData},
    Renderer,
};

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

    // TODO: figure out how to best use rayon here
    // this is currently very slow
    for (pos, chunk) in &terrain.chunks {
        if system.terrain_render_data.chunks.get(pos).is_none() {
            let mesh = mesh::create_chunk_mesh(chunk, *pos, terrain, blocks);
            for dir in &[
                Vec2::new(0, 1),
                Vec2::new(0, -1),
                Vec2::new(1, 0),
                Vec2::new(-1, 0),
            ] {
                let neighbor_pos = *pos + *dir;
                if let Some(neighbor_chunk) = terrain.chunks.get(&neighbor_pos) {
                    let mesh =
                        mesh::create_chunk_mesh(neighbor_chunk, neighbor_pos, terrain, blocks);
                    system.terrain_render_data.chunks.insert(
                        neighbor_pos,
                        TerrainRenderData {
                            buffer: system.renderer.create_vertex_buffer(&mesh),
                        },
                    );
                }
            }
            system.terrain_render_data.chunks.insert(
                *pos,
                TerrainRenderData {
                    buffer: system.renderer.create_vertex_buffer(&mesh),
                },
            );
        }
    }
    // log::info!("Meshing took {}ms", time.elapsed().as_millis());

    ok()
}
