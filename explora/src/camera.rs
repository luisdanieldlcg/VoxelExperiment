use vek::{Mat4, Vec2, Vec3};

const Z_NEAR: f32 = 0.1;
const Z_FAR: f32 = 1000.0;

pub struct Plane {
    pub normal: Vec3<f32>,
    pub distance: f32,
}

impl Plane {
    pub fn new(point: Vec3<f32>, normal: Vec3<f32>) -> Self {
        Self {
            normal: normal.normalized(),
            distance: point.dot(normal),
        }
    }
}

pub struct Matrices {
    pub view: Mat4<f32>,
    pub proj: Mat4<f32>,
}

/// Represents a camera in 3D space.
pub struct Camera {
    /// The position of the camera in world space.
    pos: Vec3<f32>,
    /// The aspect ratio of the camera.
    aspect: f32,
    /// The field of view of the camera in degrees.
    fov: f32,
    /// The rotation of the camera in radians.
    /// The x component is the yaw, the y component is the pitch.
    ///
    /// The pitch is how much we are looking up or down.
    /// The yaw is how much we are looking left or right.
    rot: Vec2<f32>,
    proj: Mat4<f32>,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            pos: Vec3::new(0.0, 257.0, 0.0),
            aspect: 1.0,
            fov: 70.0,
            rot: Vec2::new(-46.0, 0.0),
            proj: Mat4::identity(),
        }
    }
}
impl Camera {
    pub fn compute_matrices(&mut self) -> Matrices {
        let view = Mat4::look_at_lh(self.pos, self.pos + self.forward(), Vec3::unit_y());
        Matrices {
            view,
            proj: self.proj,
        }
    }

    pub fn move_by(&mut self, dx: f32, dy: f32, dz: f32) {
        self.pos += dz * self.forward_xz() + -dx * self.right() + Vec3::unit_y() * dy;
    }

    pub fn rotate_by(&mut self, dx: f32, dy: f32) {
        // 2π is a full rotation, so we need to clamp the yaw to 0..2π
        self.rot.x = (self.rot.x + dx).rem_euclid(std::f32::consts::TAU);
        // We clamp the pitch to -π/2..π/2
        self.rot.y = (self.rot.y - dy).clamp(
            -std::f32::consts::FRAC_PI_2 + 0.0001,
            std::f32::consts::FRAC_PI_2 - 0.0001,
        );
    }

    pub fn forward(&self) -> Vec3<f32> {
        Vec3::new(
            f32::cos(self.rot.x) * f32::cos(self.rot.y),
            f32::sin(self.rot.y),
            -f32::sin(self.rot.x) * f32::cos(self.rot.y),
        )
        .normalized()
    }

    pub fn forward_xz(&self) -> Vec3<f32> {
        Vec3::new(f32::cos(self.rot.x), 0.0, -f32::sin(self.rot.x)).normalized()
    }

    pub fn right(&self) -> Vec3<f32> {
        self.forward().cross(Vec3::unit_y()).normalized()
    }

    pub fn set_aspect_ratio(&mut self, aspect: f32) {
        self.aspect = aspect;
        self.rebuild_projection();
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
        self.rebuild_projection();
    }

    pub fn fov(&self) -> f32 {
        self.fov
    }

    pub fn orientation(&self) -> &str {
        let forward: Vec3<f32> = self.forward();
        let (x, y, z) = forward.map(|f| f.abs()).into_tuple();
        if x >= y && x >= z {
            return if forward.x > 0.0 { "West" } else { "East" };
        }
        if y >= z {
            return if forward.y > 0.0 { "Up" } else { "Down" };
        }
        if forward.z > 0.0 {
            "North"
        } else {
            "South"
        }
    }

    pub fn pos(&self) -> Vec3<f32> {
        self.pos
    }

    fn rebuild_projection(&mut self) {
        self.proj = Mat4::perspective_lh_no(self.fov.to_radians(), self.aspect, Z_NEAR, Z_FAR)
    }
}
