use common::{chunk::Chunk, dir::Direction, resources::TerrainMap};
use vek::{Vec2, Vec3};

use crate::vertex::TerrainVertex;

pub fn create_chunk_mesh(
    chunk: &Chunk,
    chunk_pos: Vec2<i32>,
    map: &TerrainMap,
) -> Vec<TerrainVertex> {
    let mut vertices = Vec::with_capacity(Chunk::SIZE.product());

    let dirt = 0;
    let grass = 2;

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

                let Some(neighbor_chunk) = map.0.get(&(neighbor_chunk_dir)) else {
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

        // North
        if render_quad(Direction::North) {
            create_quad(
                &mut vertices,
                world_pos + Vec3::unit_z(),
                Vec3::unit_y(),
                Vec3::unit_x(),
                dirt,
            );
        }

        // South
        if render_quad(Direction::South) {
            create_quad(
                &mut vertices,
                world_pos,
                Vec3::unit_x(),
                Vec3::unit_y(),
                dirt,
            );
        }

        // East
        if render_quad(Direction::East) {
            create_quad(
                &mut vertices,
                world_pos + Vec3::unit_x(),
                Vec3::unit_z(),
                Vec3::unit_y(),
                dirt,
            );
        }

        // West
        if render_quad(Direction::West) {
            create_quad(
                &mut vertices,
                world_pos,
                Vec3::unit_y(),
                Vec3::unit_z(),
                dirt,
            );
        }
        // Bottom
        if render_quad(Direction::Down) {
            create_quad(
                &mut vertices,
                world_pos,
                Vec3::unit_z(),
                Vec3::unit_x(),
                dirt,
            );
        }

        // Top
        if render_quad(Direction::Up) {
            create_quad(
                &mut vertices,
                world_pos + Vec3::unit_y(),
                Vec3::unit_x(),
                Vec3::unit_z(),
                grass,
            );
        }
    }
    vertices
}

fn create_quad(
    mesh: &mut Vec<TerrainVertex>,
    origin: Vec3<i32>,
    unit_x: Vec3<i32>,
    unit_y: Vec3<i32>,
    id: u32,
) {
    mesh.push(TerrainVertex::new(origin, id));
    mesh.push(TerrainVertex::new(origin + unit_x, id));
    mesh.push(TerrainVertex::new(origin + unit_x + unit_y, id));
    mesh.push(TerrainVertex::new(origin + unit_y, id));
}
