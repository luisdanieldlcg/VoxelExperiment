use common::{resources::DeltaTime, state::SysResult};
use explora::{
    camera::Camera,
    event::{Events, WindowEvent},
    input::{GameInput, Input},
};

use apecs::*;
use log::info;
use render::{GpuGlobals, Renderer};
use vek::Vec3;

#[derive(CanFetch)]
pub struct SceneSystem<'a> {
    renderer: Write<Renderer, NoDefault>,
    camera: Query<&'a mut Camera>,
    events: Read<Events<WindowEvent>>,
    delta: Read<DeltaTime>,
    input: Read<Input>,
}

pub fn scene_update_system(mut scene: SceneSystem) -> SysResult {
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
            WindowEvent::Input(input, pressed) => {},
        }
    }
    ok()
}
