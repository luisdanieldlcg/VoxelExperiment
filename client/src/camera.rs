use vek::{Mat4, Vec2, Vec3};

const Z_NEAR: f32 = 0.1;
const Z_FAR: f32 = 1000.0;

pub struct Matrices {
    pub view: Mat4<f32>,
    pub proj: Mat4<f32>,
}

pub struct Camera {
    pos: Vec3<f32>,
    pub forward: Vec3<f32>,
    rotation: Vec2<f32>,
    aspect: f32,
    fov_rad: f32,
    proj: Mat4<f32>,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        Self {
            pos: Vec3::new(0.0, 0.0, -1.0),
            forward: Vec3::new(0.0, 0.0, 1.0),
            rotation: Vec2::zero(),
            aspect,
            fov_rad: std::f32::consts::FRAC_PI_4, // 45deg
            proj: Mat4::identity(),
        }
    }

    pub fn rotate_by(&mut self, dx: f32, dy: f32) {
        self.rotation.x = (self.rotation.x + dx).rem_euclid(std::f32::consts::TAU);

        // We clamp the pitch to -π/2..π/2
        self.rotation.y = (self.rotation.y - dy).clamp(
            -std::f32::consts::FRAC_PI_2 + 0.0001,
            std::f32::consts::FRAC_PI_2 - 0.0001,
        );
        self.forward = self.forward();
    }

    pub fn move_by(&mut self, dx: f32, dy: f32, dz: f32) {
        self.pos += dz * self.forward_xz() + -dx * self.right() + Vec3::unit_y() * dy;
    }

    pub fn compute_matrices(&mut self) -> Matrices {
        let view = Mat4::look_at_lh(self.pos, self.pos + self.forward, Vec3::unit_y());
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
        self.aspect = aspect;
        self.rebuild_projection();
    }

    pub fn set_fov(&mut self, fov_radians: f32) {
        self.fov_rad = fov_radians;
        self.rebuild_projection();
    }

    pub fn fov(&self) -> f32 {
        self.fov_rad
    }

    pub fn rebuild_projection(&mut self) {
        self.proj = Mat4::perspective_lh_no(self.fov_rad, self.aspect, Z_NEAR, Z_FAR)
    }
}
