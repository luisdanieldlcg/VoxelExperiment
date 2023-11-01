use common::{chunk::Chunk, resources::TerrainMap, state::SysResult};
use render::{Renderer, TerrainRenderData};
use vek::Vec2;

use apecs::*;

#[derive(CanFetch)]
pub struct TerrainSystem {
    renderer: Write<Renderer, NoDefault>,
    terrain_map: Write<TerrainMap>,
    terrain_render_data: Write<TerrainRenderData, NoDefault>,
}

pub fn terrain_system_setup(mut system: TerrainSystem) -> SysResult {
    for x in -1..1 {
        for z in -1..1 {
            let pos = Vec2::new(x, z);
            let chunk = Chunk::generate(pos);
            system.terrain_map.0.insert(pos, chunk);
        }
    }

    let mut mesh_work = vec![];

    for (pos, chunk) in system.terrain_map.0.iter() {
        let mesh = render::mesh::create_chunk_mesh(chunk, *pos, system.terrain_map.inner());
        mesh_work.extend(mesh);
    }

    *system.terrain_render_data = TerrainRenderData {
        buffer: Some(system.renderer.create_vertex_buffer(&mesh_work)),
        wireframe_enabled: false,
    };
    end()
}
