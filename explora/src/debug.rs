use apecs::{ok, NoDefault, Read, Write};
use common::{block, resources::TerrainMap, SysResult};
use vek::{LineSegment3, Vec3};
use winit::event::MouseButton;

use crate::{
    camera::Camera,
    input::Input,
    ray::raycast,
    render::{mesh, resources::DebugRender, Renderer},
};

pub const DEBUG_SYSTEM: &str = "debug";

pub struct BoxCollider {
    pub center: Vec3<f32>,
    pub size: Vec3<f32>,
}

pub fn debug_update_system(
    (mut renderer, mut debug_render, camera, input, terrain): (
        Write<Renderer, NoDefault>,
        Write<DebugRender>,
        Read<Camera>,
        Read<Input>,
        Read<TerrainMap>,
    ),
) -> SysResult {
    if input.is_button_down(MouseButton::Left) {
        let center = Vec3::new(15.0, 65.0, -74.0);
        let size = Vec3::new(1.0, 1.0, 1.0);
        let outline_color = [0.7, 0.2, 0.2, 1.0];

        let origin = camera.pos();
        let dir = camera.forward();
        let distance = 5.0;
        raycast(origin, dir, distance);

        let color = [1.0, 0.0, 0.0, 1.0];

        let segment = LineSegment3 {
            start: origin,
            end: origin + dir * distance,
        };

        let mesh = mesh::debug::box_along_line(segment, color, 0.1, 0.1);
        mesh::debug::box_wireframe_mesh(center, outline_color, size);
        debug_render
            .mesh
            .push(renderer.create_debug_buffer(mesh.vertices()));

        let mut origin = origin;
        let mut point_on_ray = origin;
        let mut dist = 0.0;
        while dist < distance {
            origin = point_on_ray.ceil();
            match terrain.get_block(origin) {
                Some(block) if block.is_solid() => {
                    let size = Vec3::new(1.0, 1.0, 1.0);
                    // blue outline
                    let outline_color = [0.0, 0.0, 1.0, 1.0];
                    let box_pos = origin - size / 2.0;
                    let mesh = mesh::debug::box_wireframe_mesh(box_pos, outline_color, size);
                    debug_render
                        .mesh
                        .push(renderer.create_debug_buffer(mesh.vertices()));
                },
                any => {
                    log::info!("any: {:?}", any);
                },
            }
            point_on_ray += dir * 0.01;
            dist += 0.1;
        }
    }
    ok()
}
