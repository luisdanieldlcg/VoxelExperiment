use common::{resources::DeltaTime, state::SysResult};
use explora::{
    camera::Camera,
    event::{Events, WindowEvent},
    input::GameInput,
};

use apecs::*;
use render::{GpuGlobals, TerrainRenderData};
use vek::Vec3;

#[derive(CanFetch)]
pub struct SceneSystem {
    camera: Query<&'static mut Camera>,
    events: Read<Events<WindowEvent>>,
    delta: Read<DeltaTime>,
    globals: Write<GpuGlobals>,
    terrain_render_data: Write<TerrainRenderData>,
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
                for camera in scene.camera.query().iter_mut() {
                    camera.rotate(cursor.x, cursor.y, scene.delta.0);
                }
            },
            WindowEvent::KeyPress(input, state) => {
                let val = *state as i32 as f32;
                match input {
                    GameInput::MoveForward => {
                        dir.z += val;
                    },
                    GameInput::MoveBackward => {
                        dir.z -= val;
                    },
                    GameInput::MoveLeft => {
                        dir.x -=val;
                    },
                    GameInput::MoveRight => {
                        dir.x += val;
                    },
                    GameInput::Jump => {
                        dir.y +=val;
                    },
                    GameInput::Sneak => {
                        dir.y -= val;
                    },
                    GameInput::ToggleWireframe => {
                        if *state {
                            scene.terrain_render_data.wireframe_enabled= !scene.terrain_render_data.wireframe_enabled;
                        }
                    },
                }
            },
        }
    }

    let mut cameras = scene.camera.query();
    for camera in cameras.iter_mut() {
        camera.update(scene.delta.0, dir);
        let matrices = camera.build_matrices();
        *scene.globals = GpuGlobals::new(matrices.view, matrices.proj);
    }
    ok()
}
