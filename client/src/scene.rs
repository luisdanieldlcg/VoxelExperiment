use vek::Vec3;

use crate::camera::{Camera, Matrices};

pub struct Scene {
    camera: Camera,
    movement_dir: Vec3<f32>,
}
// TODO: make this configurable
const FLY_CAMERA_SPEED: f32 = 3.0;

impl Scene {
    pub fn new() -> Self {
        let camera = Camera::new();
        Self {
            camera,
            movement_dir: Vec3::zero(),
        }
    }

    pub fn set_movement_dir(&mut self, dir: Vec3<f32>) {
        self.movement_dir = dir;
    }

    pub fn look(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.camera.rotate_by(delta_yaw * 0.05, delta_pitch * 0.05);
    }

    pub fn resize(&mut self, aspect: f32) {
        self.camera.set_aspect_ratio(aspect);
    }

    pub fn update(&mut self, dt: f32) -> Matrices {
        let dx = self.movement_dir.x * FLY_CAMERA_SPEED * dt;
        let dy = self.movement_dir.y * FLY_CAMERA_SPEED * dt;
        let dz = self.movement_dir.z * FLY_CAMERA_SPEED * dt;
        self.camera.move_by(dx, dy, dz);
        self.camera.compute_matrices()
    }
}
