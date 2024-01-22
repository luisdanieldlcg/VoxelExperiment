pub struct GameplaySettings {
    pub mouse_sensitivity: u32,
    pub free_camera_speed: f32,
}

impl Default for GameplaySettings {
    fn default() -> Self {
        Self {
            // 100% means default sensitivity
            mouse_sensitivity: 100,
            free_camera_speed: 20.0,
        }
    }
}
