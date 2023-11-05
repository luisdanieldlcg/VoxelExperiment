use core::{chunk::Chunk, dir::Direction, resources::TerrainMap};
use render::vertex::TerrainVertex;
use vek::{Vec2, Vec3};

use crate::block::BlockMap;

pub fn create_chunk_mesh(
    chunk: &Chunk,
    chunk_pos: Vec2<i32>,
    terrain_map: &TerrainMap,
    block_map: &BlockMap,
) -> Vec<TerrainVertex> {
    let mut vertices = Vec::with_capacity(Chunk::SIZE.product());
    for pos in chunk.iter() {
        let render_quad = |direction: Direction| {
            let dir = direction.vec(); // The direction of the face we are checking for render
            let adjacent_pos = pos + dir; // The pos of the adjacent block

            if Chunk::out_of_bounds(adjacent_pos) {
                // If the adjacent block is out of bounds
                // it means we are at the edge of the chunk
                if matches!(direction, Direction::Up) || matches!(direction, Direction::Down) {
                    // If the direction is up or down we can render the quad
                    // Since we have no chunks above or below
                    return true;
                }

                // Now we have to check if there is a chunk adjacent to this one
                let neighbor_chunk_dir = Vec2::new(chunk_pos.x + dir.x, chunk_pos.y + dir.z);

                let Some(neighbor_chunk) = terrain_map.0.get(&(neighbor_chunk_dir)) else {
                    // If there is no adjacent chunk we have to render the quad
                    // because it is a border of the chunk
                    return true;
                };

                // map out of bound adj block pos to neighbor local pos
                let neighbor_block_in_border = Vec3::new(
                    if adjacent_pos.x < 0 {
                        Chunk::SIZE.x as i32 - 1
                    } else if adjacent_pos.x >= Chunk::SIZE.x as i32 {
                        0
                    } else {
                        adjacent_pos.x
                    },
                    adjacent_pos.y,
                    if adjacent_pos.z < 0 {
                        Chunk::SIZE.z as i32 - 1
                    } else if adjacent_pos.z >= Chunk::SIZE.z as i32 {
                        0
                    } else {
                        adjacent_pos.z
                    },
                );
                // Check if the adjacent block is air or not in the map
                return match neighbor_chunk.get(neighbor_block_in_border) {
                    Some(id) => id.is_air(),
                    None => true,
                };
            }
            // The adjacent block is within the bounds of this chunk
            // render only if the adjacent block is not there e.g air or not in the map
            match chunk.get(adjacent_pos) {
                Some(id) => id.is_air(),
                None => true,
            }
        };

        let world_pos = Vec3::new(
            pos.x + chunk_pos.x * Chunk::SIZE.x as i32,
            pos.y,
            pos.z + chunk_pos.y * Chunk::SIZE.z as i32,
        );

        let id = match chunk.get(pos) {
            Some(id) => id,
            None => continue,
        };

        if id.is_air() {
            continue;
        }

        let block = block_map
            .0
            .get(&id)
            .unwrap_or_else(|| panic!("The block with id: {} is not registered", id as u32));

        let top = block.textures.top;
        let bottom = block.textures.bottom;
        let side = block.textures.side;
        // North
        if render_quad(Direction::North) {
            vertices.push(TerrainVertex::new(
                world_pos + Vec3::unit_x() + Vec3::unit_z(),
                side,
            ));
            vertices.push(TerrainVertex::new(world_pos + Vec3::unit_z(), side));
            vertices.push(TerrainVertex::new(
                world_pos + Vec3::unit_z() + Vec3::unit_y(),
                side,
            ));
            vertices.push(TerrainVertex::new(
                world_pos + Vec3::unit_z() + Vec3::unit_x() + Vec3::unit_y(),
                side,
            ));
        }

        // South
        if render_quad(Direction::South) {
            vertices.push(TerrainVertex::new(world_pos, side));
            vertices.push(TerrainVertex::new(world_pos + Vec3::unit_x(), side));
            vertices.push(TerrainVertex::new(
                world_pos + Vec3::unit_x() + Vec3::unit_y(),
                side,
            ));
            vertices.push(TerrainVertex::new(world_pos + Vec3::unit_y(), side));
        }

        // East
        if render_quad(Direction::East) {
            vertices.push(TerrainVertex::new(world_pos + Vec3::unit_x(), side));
            vertices.push(TerrainVertex::new(
                world_pos + Vec3::unit_x() + Vec3::unit_z(),
                side,
            ));
            vertices.push(TerrainVertex::new(
                world_pos + Vec3::unit_x() + Vec3::unit_z() + Vec3::unit_y(),
                side,
            ));
            vertices.push(TerrainVertex::new(
                world_pos + Vec3::unit_x() + Vec3::unit_y(),
                side,
            ));
        }

        // West
        if render_quad(Direction::West) {
            vertices.push(TerrainVertex::new(world_pos + Vec3::unit_z(), side));
            vertices.push(TerrainVertex::new(world_pos, side));
            vertices.push(TerrainVertex::new(world_pos + Vec3::unit_y(), side));
            vertices.push(TerrainVertex::new(
                world_pos + Vec3::unit_z() + Vec3::unit_y(),
                side,
            ));
        }
        // Bottom
        if render_quad(Direction::Down) {
            vertices.push(TerrainVertex::new(world_pos, bottom));
            vertices.push(TerrainVertex::new(world_pos + Vec3::unit_z(), bottom));
            vertices.push(TerrainVertex::new(
                world_pos + Vec3::unit_x() + Vec3::unit_z(),
                bottom,
            ));
            vertices.push(TerrainVertex::new(world_pos + Vec3::unit_x(), bottom));
        }

        // Top
        if render_quad(Direction::Up) {
            vertices.push(TerrainVertex::new(world_pos + Vec3::unit_y(), top));
            vertices.push(TerrainVertex::new(
                world_pos + Vec3::unit_y() + Vec3::unit_x(),
                top,
            ));
            vertices.push(TerrainVertex::new(
                world_pos + Vec3::unit_y() + Vec3::unit_x() + Vec3::unit_z(),
                top,
            ));
            vertices.push(TerrainVertex::new(
                world_pos + Vec3::unit_y() + Vec3::unit_z(),
                top,
            ));
        }
    }
    vertices
}
