use vek::{Mat4, Vec2, Vec3};

const Z_NEAR: f32 = 0.1;
const Z_FAR: f32 = 1000.0;

pub struct Matrices {
    pub view: Mat4<f32>,
    pub proj: Mat4<f32>,
}

pub struct Camera {
    pos: Vec3<f32>,
    target: Vec3<f32>,
    aspect: f32,
    fov: f32,
    /// The rotation of the camera in radians.
    /// The x component is the yaw, the y component is the pitch.
    rot: Vec2<f32>,
    pub speed: f32,
    pub sensitivity: f32,
    proj: Mat4<f32>,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        Self {
            pos: Vec3::new(0.0, 257.0, -2.0),
            target: Vec3::new(0.0, 0.0, 0.0),
            aspect,
            fov: 70.0,
            rot: Vec2::new(-46.0, 0.0),
            speed: 20.0,
            sensitivity: 0.1,
            proj: Mat4::identity(),
        }
    }

    pub fn compute_matrices(&self) -> Matrices {
        Matrices {
            view: Mat4::look_at_lh(self.pos, self.pos + self.target, Vec3::unit_y()),
            proj: self.proj,
        }
    }

    pub fn rotate(&mut self, dx: f32, dy: f32) {
        let sensitivity = self.sensitivity;
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

    pub fn update(&mut self, dt: f32, dir: Vec3<f32>) {
        let forward = self.forward();
        let right = self.right();
        let dx = right * -dir.x * self.speed * dt;
        let dy = Vec3::unit_y() * dir.y * self.speed * dt;
        let dz = forward * dir.z * self.speed * dt;
        self.pos += dx + dy + dz;
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
        let (x, y, z) = self.target.map(|f| f.abs()).into_tuple();
        if x >= y && x >= z {
            return if self.target.x > 0.0 { "West" } else { "East" };
        }
        if y >= z {
            return if self.target.y > 0.0 { "Up" } else { "Down" };
        }
        if self.target.z > 0.0 {
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
