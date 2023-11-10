use core::{event::Events, resources::DeltaTime, SysResult};

use apecs::*;

use render::{resources::TerrainRender, GpuGlobals, Renderer};
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
    globals: Write<GpuGlobals>,
    terrain_render_data: Write<TerrainRender>,
    window: Write<Window, NoDefault>,
    renderer: Write<Renderer, NoDefault>,
}

pub fn scene_update_system(mut scene: SceneSystem) -> SysResult {
    let mut dir = Vec3::<f32>::zero();

    for event in &scene.events.events {
        match event {
            WindowEvent::Close => {},
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
            WindowEvent::KeyPress(input) => {
                let val = 1.0;
                match input {
                    GameInput::MoveForward => {
                        dir.z += val;
                    },
                    GameInput::MoveBackward => {
                        dir.z -= val;
                    },
                    GameInput::MoveLeft => {
                        dir.x -= val;
                    },
                    GameInput::MoveRight => {
                        dir.x += val;
                    },
                    GameInput::Jump => {
                        dir.y += val;
                    },
                    GameInput::Sneak => {
                        dir.y -= val;
                    },
                    _ => (),
                }
            },
            WindowEvent::JustPressed(key) => {
                if let GameInput::ToggleWireframe = key {
                    scene.terrain_render_data.wireframe = !scene.terrain_render_data.wireframe;
                }
                if let GameInput::ToggleCursor = key {
                    scene.window.toggle_cursor();
                }
            },
        }
    }

    let mut cameras = scene.camera.query();
    for camera in cameras.iter_mut() {
        camera.update(scene.delta.0, dir);
        let matrices = camera.build_matrices();
        let sun_pos = Vec3::new(15.0, 320.0, 15.0);
        let new_globals = GpuGlobals::new(
            matrices.view,
            matrices.proj,
            sun_pos,
            scene.globals.enable_lighting,
        );
        *scene.globals = new_globals;
        scene.renderer.write_globals(*scene.globals);
    }
    ok()
}
