use common::{
    ecs::{NoDefault, Query, Read, ShouldContinue, Write},
    resources::DeltaTime,
    state::SysResult,
};

use render::{Globals, Renderer};
use vek::{Mat4, Vec2, Vec3};

use crate::input::Input;

const Z_NEAR: f32 = 0.1;
const Z_FAR: f32 = 1000.0;

pub struct Matrices {
    view: Mat4<f32>,
    proj: Mat4<f32>,
}

pub struct Camera {
    pos: Vec3<f32>,
    target: Vec3<f32>,
    aspect: f32,
    fov: f32,
    /// The rotation of the camera in radians.
    /// The x component is the yaw, the y component is the pitch.
    rot: Vec2<f32>,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        Self {
            pos: Vec3::new(0.0, 0.0, -2.0),
            target: Vec3::new(0.0, 0.0, 0.0),
            aspect,
            fov: 70.0,
            rot: Vec2::new(-46.0, 0.0),
        }
    }

    pub fn build_matrices(&mut self) -> Matrices {
        let view: Mat4<f32> = Mat4::look_at_lh(self.pos, self.pos + self.target, Vec3::unit_y());
        let proj: Mat4<f32> =
            Mat4::perspective_lh_no(self.fov.to_radians(), self.aspect, Z_NEAR, Z_FAR);
        Matrices { view, proj }
    }

    pub fn rotate(&mut self, dx: f32, dy: f32, _dt: f32) {
        let sensitivity = 0.1;
        let offset_x = dx * sensitivity;
        let offset_y = dy * sensitivity;

        self.rot.x += offset_x.to_radians();
        self.rot.y += -offset_y.to_radians();

        self.rot.y = self.rot.y.clamp(
            -std::f32::consts::FRAC_PI_2 + 0.0001,
            std::f32::consts::FRAC_PI_2 - 0.0001,
        );

        let (yaw_sin, yaw_cos) = self.rot.x.sin_cos();
        let (pitch_sin, pitch_cos) = self.rot.y.sin_cos();

        // yaw_sin z goes negative for left handed coordinate system
        self.target = Vec3::new(yaw_cos * pitch_cos, pitch_sin, -yaw_sin * pitch_cos).normalized();
    }

    pub fn forward(&self) -> Vec3<f32> {
        Vec3::new(f32::cos(self.rot.x), 0.0, -f32::sin(self.rot.x)).normalized()
    }
    pub fn right(&self) -> Vec3<f32> {
        Vec3::new(f32::sin(self.rot.x), 0.0, f32::cos(self.rot.x)).normalized()
    }

    pub fn set_aspect_ratio(&mut self, aspect: f32) {
        self.aspect = aspect;
    }
}

pub fn camera_system(
    (mut renderer, cameras, delta, input): (
        Write<Renderer, NoDefault>,
        Query<&mut Camera>,
        Read<DeltaTime>,
        Read<Input>,
    ),
) -> SysResult {
    let mut cameras = cameras.query();
    for camera in cameras.iter_mut() {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;

        if input.is_key_down(winit::keyboard::KeyCode::KeyW) {
            z += 1.0;
        }
        if input.is_key_down(winit::keyboard::KeyCode::KeyS) {
            z -= 1.0;
        }
        if input.is_key_down(winit::keyboard::KeyCode::KeyA) {
            x -= 1.0;
        }
        if input.is_key_down(winit::keyboard::KeyCode::KeyD) {
            x += 1.0;
        }
        if input.is_key_down(winit::keyboard::KeyCode::Space) {
            y += 1.0;
        }
        if input.is_key_down(winit::keyboard::KeyCode::ShiftLeft) {
            y -= 1.0;
        }

        let forward = camera.forward();
        let right = camera.right();

        let dir = Vec3::new(x, y, z);

        let speed = 2.0;
        let dx = right * -dir.x * speed * delta.0;
        let dy = Vec3::unit_y() * dir.y * speed * delta.0;
        let dz = forward * dir.z * speed * delta.0;
        camera.pos += dx + dy + dz;

        let matrices = camera.build_matrices();
        let globals = Globals::new(matrices.view, matrices.proj);
        renderer.write_globals(globals);
    }
    Ok(ShouldContinue::Yes)
}
