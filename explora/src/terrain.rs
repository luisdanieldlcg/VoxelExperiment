use core::{resources::TerrainMap, SysResult};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use render::{
    resources::{TerrainRender, TerrainRenderData},
    vertex::TerrainVertex,
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
    let time = std::time::Instant::now();

    // this is currently very slow
    let iterator = terrain
        .chunks
        .par_iter()
        .filter_map(|(pos, chunk)| {
            if system.terrain_render_data.chunks.get(pos).is_none() {
                // TODO: reuse mesh vector.
                let mut meshes = vec![];
                let mesh = mesh::create_chunk_mesh(chunk, *pos, terrain, blocks);
                let buffer = system.renderer.create_vertex_buffer(&mesh);
                meshes.push((buffer, *pos));

                // remesh neighbors
                let neighbors = [
                    Vec2::new(0, 1),
                    Vec2::new(0, -1),
                    Vec2::new(1, 0),
                    Vec2::new(-1, 0),
                ];

                // generate neighbor meshes
                for dir in &neighbors {
                    let neighbor_pos = *pos + *dir;
                    if let Some(neighbor_chunk) = terrain.chunks.get(&neighbor_pos) {
                        let mesh =
                            mesh::create_chunk_mesh(neighbor_chunk, neighbor_pos, terrain, blocks);
                        let buffer = system.renderer.create_vertex_buffer(&mesh);
                        meshes.push((buffer, neighbor_pos));
                    }
                }
                return Some(meshes);
            }
            None
        })
        .flatten()
        .collect::<Vec<_>>();

    for (buffer, chunk_pos) in iterator {
        system
            .renderer
            .check_index_buffer::<TerrainVertex>(buffer.len() as usize);
        system
            .terrain_render_data
            .chunks
            .insert(chunk_pos, TerrainRenderData { buffer });
    }

    log::info!("Meshing took {}ms", time.elapsed().as_millis());

    ok()
}
