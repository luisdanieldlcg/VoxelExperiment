use crate::render::vertex::TerrainVertex;
use common::{chunk::Chunk, dir::Direction, resources::TerrainMap};
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
        let origin = pos.map(|f| f as u32);
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

                let Some(neighbor_chunk) = terrain_map.chunks.get(&(neighbor_chunk_dir)) else {
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
            let normal = Direction::North.vec();
            quad(
                &mut vertices,
                origin + Vec3::unit_z(),
                Vec3::unit_y(),
                Vec3::unit_x(),
                normal,
                side,
            );
        }

        // South
        if render_quad(Direction::South) {
            let normal = Direction::South.vec();
            quad(
                &mut vertices,
                origin,
                Vec3::unit_x(),
                Vec3::unit_y(),
                normal,
                side,
            );
            
        }

        // East
        if render_quad(Direction::East) {
            let normal = Direction::East.vec();
            quad(
                &mut vertices,
                origin + Vec3::unit_x(),
                Vec3::unit_z(),
                Vec3::unit_y(),
                normal,
                side,
            );
            
        }

        // West
        if render_quad(Direction::West) {
            let normal = Direction::West.vec();
            quad(
                &mut vertices,
                origin,
                Vec3::unit_y(),
                Vec3::unit_z(),
                normal,
                side,
            );
      
        }
        // Bottom
        if render_quad(Direction::Down) {
            let normal = Direction::Down.vec();
            quad(
                &mut vertices,
                origin,
                Vec3::unit_z(),
                Vec3::unit_x(),
                normal,
                bottom,
            );
        }

        // Top
        if render_quad(Direction::Up) {
            let normal = Direction::Up.vec();
            quad(
                &mut vertices,
                origin + Vec3::unit_y(),
                Vec3::unit_x(),
                Vec3::unit_z(),
                normal,
                top,
            );
        
        }
    }
    vertices
}

fn quad(
    mesh: &mut Vec<TerrainVertex>,
    origin: Vec3<u32>,
    unit_x: Vec3<u32>,
    unit_y: Vec3<u32>,
    normal: Vec3<i32>,
    id: u16,
) {
    mesh.push(TerrainVertex::new(origin, id, normal));
    mesh.push(TerrainVertex::new(origin + unit_x, id, normal));
    mesh.push(TerrainVertex::new(origin + unit_x + unit_y, id, normal));
    mesh.push(TerrainVertex::new(origin + unit_y, id, normal));
}