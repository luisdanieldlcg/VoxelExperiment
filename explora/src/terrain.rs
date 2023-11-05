use core::{chunk::Chunk, resources::TerrainMap, SysResult};
use noise::Perlin;
use rand::Rng;
use render::{Renderer, TerrainRenderData};
use vek::Vec2;

use apecs::*;

use crate::{block::BlockMap, mesh};

#[derive(CanFetch)]
pub struct TerrainSystem {
    renderer: Write<Renderer, NoDefault>,
    terrain_map: Write<TerrainMap>,
    block_map: Read<BlockMap>,
    terrain_render_data: Write<TerrainRenderData, NoDefault>,
}

pub fn terrain_system_setup(mut system: TerrainSystem) -> SysResult {
    let terrain = system.terrain_map.inner_mut();

    let blocks = system.block_map.inner();
    let seed = rand::thread_rng().gen_range(0..100);
    let noise = Perlin::new(seed);
    let radius = 2;
    for x in -radius..radius {
        for z in -radius..radius {
            let pos = Vec2::new(x, z);
            let chunk = Chunk::generate(noise, pos);
            terrain.0.insert(pos, chunk);
        }
    }
    let mut mesh_work = Vec::with_capacity(Chunk::SIZE.product());

    for (pos, chunk) in terrain.0.iter() {
        let mesh = mesh::create_chunk_mesh(chunk, *pos, terrain, blocks);
        mesh_work.extend(mesh);
    }
    *system.terrain_render_data = TerrainRenderData {
        buffer: Some(system.renderer.create_vertex_buffer(&mesh_work)),
        wireframe_enabled: false,
    };
    end()
}
