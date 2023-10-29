use common::chunk::Chunk;
use vek::Vec3;

use crate::vertex::TerrainVertex;

pub fn create_chunk_mesh(chunk: &Chunk) -> Vec<TerrainVertex> {
    let mut vertices = Vec::with_capacity(Chunk::SIZE.product());

    let dirt = 0;
    let grass = 2;

    for pos in chunk.iter() {
        // -x
        if !Chunk::is_within_bounds(pos - Vec3::unit_x()) {
            create_quad(&mut vertices, pos, Vec3::unit_y(), Vec3::unit_z(), dirt);
        }
        // +x
        if !Chunk::is_within_bounds(pos + Vec3::unit_x()) {
            create_quad(
                &mut vertices,
                pos + Vec3::unit_x(),
                Vec3::unit_z(),
                Vec3::unit_y(),
                dirt,
            );
        }
        // -y
        if !Chunk::is_within_bounds(pos - Vec3::unit_y()) {
            create_quad(&mut vertices, pos, Vec3::unit_z(), Vec3::unit_x(), dirt);
        }

        // +y
        if !Chunk::is_within_bounds(pos + Vec3::unit_y()) {
            create_quad(
                &mut vertices,
                pos + Vec3::unit_y(),
                Vec3::unit_x(),
                Vec3::unit_z(),
                grass,
            );
        }

        // -z
        if !Chunk::is_within_bounds(pos - Vec3::unit_z()) {
            create_quad(&mut vertices, pos, Vec3::unit_x(), Vec3::unit_y(), dirt);
        }

        // +z
        if !Chunk::is_within_bounds(pos + Vec3::unit_z()) {
            create_quad(
                &mut vertices,
                pos + Vec3::unit_z(),
                Vec3::unit_y(),
                Vec3::unit_x(),
                dirt,
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
