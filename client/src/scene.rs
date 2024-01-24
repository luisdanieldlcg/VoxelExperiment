use vek::Vec3;

use crate::camera::{Camera, Matrices};

pub struct Scene {
    camera: Camera,
    movement_dir: Vec3<f32>,
}

impl Scene {
    pub fn new(aspect: f32) -> Self {
        let camera = Camera::new(aspect);
        Self {
            camera,
            movement_dir: Vec3::zero(),
        }
    }

    pub fn set_movement_dir(&mut self, dir: Vec3<f32>) {
        self.movement_dir = dir;
    }

    pub fn look(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.camera.rotate_by(delta_yaw * 0.005, delta_pitch * 0.005);
    }

    pub fn resize(&mut self, aspect: f32) {
        self.camera.set_aspect_ratio(aspect);
    }
    pub fn update(&mut self, dt: f32) -> Matrices {
        // update camera
        let speed = 1.0f32;
        let dx = self.movement_dir.x * speed * dt;
        let dy = self.movement_dir.y * speed * dt;
        let dz = self.movement_dir.z * speed * dt;

        self.camera.move_by(dx, dy, dz);
        self.camera.compute_matrices()
    }
}
