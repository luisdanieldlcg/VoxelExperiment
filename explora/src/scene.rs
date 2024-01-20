use common::{chunk::Chunk, event::Events, resources::DeltaTime, SysResult};

use apecs::*;

use crate::{
    input::Input,
    render::{atlas::BlockAtlas, resources::TerrainRender, ChunkPos, Renderer, Uniforms},
};
use vek::Vec3;

use crate::{
    camera::Camera,
    input::GameInput,
    window::{Window, WindowEvent},
};

#[derive(CanFetch)]
pub struct SceneSystem {
    camera: Query<&'static mut Camera>,
    events: Read<Events<WindowEvent>>,
    delta: Read<DeltaTime>,
    globals: Write<Uniforms>,
    terrain_render_data: Write<TerrainRender>,
    window: Write<Window, NoDefault>,
    renderer: Write<Renderer, NoDefault>,
    input: Read<Input>,
    block_atlas: Read<BlockAtlas, NoDefault>,
}

pub fn scene_update_system(mut scene: SceneSystem) -> SysResult {
    let dir = scene.input.move_direction();

    if scene.input.just_pressed(GameInput::ToggleCursor) {
        scene.window.toggle_cursor();
    }

    if scene.input.just_pressed(GameInput::ToggleWireframe) {
        scene.terrain_render_data.wireframe = !scene.terrain_render_data.wireframe;
    }

    for event in &scene.events.events {
        match event {
            WindowEvent::Resize(size) => {
                for camera in scene.camera.query().iter_mut() {
                    camera.set_aspect_ratio(size.x as f32 / size.y as f32);
                }
            },
            WindowEvent::CursorMove(cursor) => {
                if scene.window.cursor_locked() {
                    // HACK: This is a hack to prevent the camera from moving around
                    // when the cursor is locked.
                    for camera in scene.camera.query().iter_mut() {
                        camera.rotate(cursor.x, cursor.y);
                    }
                }
            },
            _ => {},
        }
    }

    let mut cameras = scene.camera.query();

    for camera in cameras.iter_mut() {
        camera.update(scene.delta.0, dir);
        let matrices = camera.compute_matrices();
        let sun_pos = Vec3::new(15.0, 300.0, 15.0);
        let new_globals = Uniforms::new(
            matrices.view,
            matrices.proj,
            sun_pos,
            scene.globals.enable_lighting,
            scene.block_atlas.atlas_size,
            scene.block_atlas.tile_size,
        );
        *scene.globals = new_globals;
        scene.renderer.write_uniforms(*scene.globals);
    }
    ok()
}
