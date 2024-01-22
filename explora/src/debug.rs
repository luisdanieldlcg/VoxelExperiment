use apecs::{ok, NoDefault, Read, Write};
use common::SysResult;
use vek::Vec3;

use crate::{camera::Camera, render::{
    mesh,
    resources::DebugRender,
    Renderer,
}};

pub const DEBUG_SYSTEM: &str = "debug";

pub fn debug_update_system(
    (mut renderer, mut debug_render,camera): (
        Write<Renderer, NoDefault>,
        Write<DebugRender>,
        Read<Camera>
    ),
) -> SysResult {

    let center = Vec3::new(15.0, 65.0, -74.0);
    let size = Vec3::new(1.0, 1.0, 1.0);
    let outline_color = [0.7, 0.2, 0.2, 1.0];
    let mesh = mesh::debug::box_wireframe_mesh(center, outline_color, size);

    debug_render.mesh = Some(renderer.create_debug_buffer(mesh.vertices()));
    ok()
}
