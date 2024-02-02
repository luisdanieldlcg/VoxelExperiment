use vek::{Mat4, Vec2, Vec3};

const Z_NEAR: f32 = 0.1;
const Z_FAR: f32 = 1000.0;

pub struct Matrices {
    pub view: Mat4<f32>,
    pub proj: Mat4<f32>,
}

pub struct Camera {
    /// The position of this camera.
    pos: Vec3<f32>,
    /// The rotation of the camera using euler angles (pitch, yaw).
    rotation: Vec2<f32>,
    /// Field of view in radians.
    fov_rad: f32,
    /// Te perspective projection
    proj: Mat4<f32>,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: Vec3::new(0.0, 0.0, -3.0),
            rotation: Vec2::new(-1.5, 0.0),
            fov_rad: std::f32::consts::FRAC_PI_4, // 45deg
            proj: Mat4::identity(),
        }
    }

    pub fn rotate_by(&mut self, dx: f32, dy: f32) {
        self.rotation.x += dx.to_radians();
        self.rotation.y += -dy.to_radians();
        self.rotation.y = self.rotation.y.clamp(
            -std::f32::consts::FRAC_PI_2 + 0.0001,
            std::f32::consts::FRAC_PI_2 - 0.0001,
        );
    }

    pub fn move_by(&mut self, dx: f32, dy: f32, dz: f32) {
        self.pos += dz * self.forward_xz() + -dx * self.right() + Vec3::unit_y() * dy;
    }

    pub fn compute_matrices(&mut self) -> Matrices {
        let view = Mat4::look_at_lh(self.pos, self.pos + self.forward(), Vec3::unit_y());
        Matrices {
            view,
            proj: self.proj,
        }
    }

    pub fn forward(&self) -> Vec3<f32> {
        Vec3::new(
            f32::cos(self.rotation.x) * f32::cos(self.rotation.y),
            f32::sin(self.rotation.y),
            -f32::sin(self.rotation.x) * f32::cos(self.rotation.y),
        )
        .normalized()
    }

    pub fn forward_xz(&self) -> Vec3<f32> {
        Vec3::new(f32::cos(self.rotation.x), 0.0, -f32::sin(self.rotation.x)).normalized()
    }

    pub fn right(&self) -> Vec3<f32> {
        self.forward().cross(Vec3::unit_y()).normalized()
    }

    pub fn set_aspect_ratio(&mut self, aspect: f32) {
        self.rebuild_projection(aspect);
    }

    pub fn rebuild_projection(&mut self, aspect: f32) {
        self.proj = Mat4::perspective_lh_no(self.fov_rad, aspect, Z_NEAR, Z_FAR)
    }
}
