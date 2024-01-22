use std::fmt::Debug;

use common::{
    resources::{TerrainConfig, TerrainMap},
    SysResult,
};

use crate::{
    camera::Camera,
    render::{
        atlas::BlockAtlas, mesh::chunk::create_chunk_mesh, resources::TerrainRender, ChunkPos,
        Renderer,
    },
};

use apecs::*;
use vek::Vec2;

use crate::block::BlockMap;

#[derive(CanFetch)]
pub struct TerrainSystem {
    renderer: Write<Renderer, NoDefault>,
    terrain_map: Write<TerrainMap>,
    block_map: Read<BlockMap, NoDefault>,
    atlas: Read<BlockAtlas, NoDefault>,
    terrain_render_data: Write<TerrainRender, NoDefault>,
}

pub const TERRAIN_CHUNK_MESH_SYSTEM: &str = "terrain_chunk_mesh";

pub fn terrain_chunk_mesh(mut system: TerrainSystem) -> SysResult {
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
            let vertices =
                create_chunk_mesh(chunk, *pos, &system.terrain_map, blocks, &system.atlas);
            let buffer = system.renderer.create_vertex_buffer(&vertices);
            let chunk_pos = ChunkPos::new(pos.x, pos.y);
            let terrain_mesh = system.renderer.create_terrain_chunk_mesh(chunk_pos, buffer);
            system.terrain_render_data.chunks.insert(*pos, terrain_mesh);
        }
    }
    ok()
}

pub const CHUNK_LOAD_SYSTEM: &str = "chunk_load";

#[derive(CanFetch)]
pub struct ChunkLoadSystem {
    terrain: Write<TerrainMap>,
    camera: Read<Camera>,
    terrain_render: Write<TerrainRender>,
    terrain_config: Read<TerrainConfig>,
}

pub fn chunk_load_system(mut system: ChunkLoadSystem) -> apecs::anyhow::Result<ShouldContinue> {
    let camera_pos = system.camera.pos();

    let chunk_radius = system.terrain_config.visible_chunk_radius as i32;
    let player_chunk_pos = Vec2::new(
        (camera_pos.x / 16.0).round() as i32,
        (camera_pos.z / 16.0).round() as i32,
    );

    // Calculate the bounding box of chunks to keep
    let min_x = player_chunk_pos.x - chunk_radius;
    let max_x = player_chunk_pos.x + chunk_radius;
    let min_z = player_chunk_pos.y - chunk_radius;
    let max_z = player_chunk_pos.y + chunk_radius;

    let mut chunks_to_remove = Vec::with_capacity(system.terrain.chunks.len());
    for (pos, _) in system.terrain.chunks.iter() {
        if pos.x < min_x || pos.x > max_x || pos.y < min_z || pos.y > max_z {
            chunks_to_remove.push(*pos);
        }
    }

    for chunk_pos in chunks_to_remove {
        system.terrain.pending_chunks.remove(&chunk_pos);
        system.terrain.chunks.remove(&chunk_pos);
        system.terrain_render.chunks.remove(&chunk_pos);
    }

    // load chunks
    for dx in -chunk_radius..=chunk_radius {
        for dz in -chunk_radius..=chunk_radius {
            let pos = Vec2::new(player_chunk_pos.x + dx, player_chunk_pos.y + dz);
            if !system.terrain.chunks.contains_key(&pos)
                && !system.terrain.pending_chunks.contains(&pos)
                && !system.terrain_render.chunks.contains_key(&pos)
            {
                system.terrain.pending_chunks.insert(pos);
            }
        }
    }
    ok()
}
