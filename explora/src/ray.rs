use vek::Vec3;

pub fn raycast(origin: Vec3<f32>, dir: Vec3<f32>, distance: f32) {
    let end = origin + dir * distance;
}
