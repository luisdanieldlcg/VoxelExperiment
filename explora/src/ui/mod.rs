use common::{
    clock::Clock,
    resources::{GameMode, Ping, TerrainConfig, TerrainMap},
    SysResult,
};

use apecs::{NoDefault, Read};

use crate::render::resources::{EguiContext, EguiSettings};
use vek::Vec2;

use crate::render::{Renderer, Uniforms};

use crate::{camera::Camera, window::Window};

pub struct EguiState {
    pub state: egui_winit::State,
}

impl EguiState {
    pub fn new(ctx: &egui::Context, window: &winit::window::Window) -> Self {
        Self {
            state: egui_winit::State::new(ctx.clone(), ctx.viewport_id(), window, None, None),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct EguiInput(egui::RawInput);

impl EguiInput {
    pub fn get(&self) -> &egui::RawInput {
        &self.0
    }

    pub fn set(&mut self, input: egui::RawInput) {
        self.0 = input;
    }
}

use apecs::*;
#[derive(CanFetch)]
pub struct EguiRenderSystem {
    egui_input: Read<EguiInput>,
    egui_config: Write<EguiSettings>,
    egui_context: Read<EguiContext>,
    clock: Read<Clock>,
    camera: Query<&'static mut Camera>,
    renderer: Write<Renderer, NoDefault>,
    window: Read<Window, NoDefault>,
    globals: Write<Uniforms>,
    ping: Read<Ping>,
    mode: Read<GameMode, NoDefault>,
    terrain_config: Write<TerrainConfig>,
    terrain: Read<TerrainMap>,
}

// This system must run before the render system
pub fn ui_debug_render_system(mut system: EguiRenderSystem) -> SysResult {
    let input = system.egui_input.get();
    system.egui_context.get().begin_frame(input.clone());

    let scale_factor = system.window.platform().scale_factor() as f32;

    *system.egui_config = EguiSettings { scale_factor };

    let mut camera = system.camera.query();
    if let Some(player_camera) = camera.find_one(0) {
        let orientation = player_camera.orientation();
        let mut camera_speed = player_camera.speed;
        let mut camera_sensitivity = player_camera.sensitivity;
        let mut camera_fov = player_camera.fov();
        let mut lighting = system.globals.enable_lighting != 0;
        egui::Window::new("Debug")
            .default_width(360.0)
            .default_height(360.0)
            .show(system.egui_context.get(), |ui| {
                ui.heading(format!("Game Mode: {:?}", *system.mode));
                ui.separator();
                ui.label(format!("Ping: {:.2}ms", system.ping.0 * 1000.0));
                ui.label(format!("FPS: {}", system.clock.fps()));
                ui.label(format!("Facing: {}", orientation));
                let pos = player_camera.pos();
                ui.label(format!(
                    "World Position: ({:.2}, {:.2}, {:.2})",
                    pos.x, pos.y, pos.z
                ));
                let chunk_pos =
                    Vec2::new((pos.x / 16.0).floor() as i32, (pos.z / 16.0).floor() as i32);

                ui.label(format!(
                    "Chunk Position: (X: {}, Z: {})",
                    chunk_pos.x, chunk_pos.y
                ));
                ui.separator();
                ui.label(format!(
                    "Graphics backend: {}",
                    system.renderer.graphics_backend
                ));
                ui.separator();
                // tweak camera speed
                ui.label("Camera speed");
                ui.add(egui::Slider::new(&mut camera_speed, 0.0..=50.0).text("speed"));
                ui.label("Camera sensitivity");
                ui.add(egui::Slider::new(&mut camera_sensitivity, 0.0..=1.0).text("sensitivity"));
                ui.label("Camera Field of View");
                ui.add(egui::Slider::new(&mut camera_fov, 0.0..=180.0).text("fov"));
                ui.separator();
                // Voxel lighting
                ui.label("Lighting");
                // add box
                ui.checkbox(&mut lighting, "Voxel Lighting".to_string());
                ui.separator();
                ui.label("Terrain");
                ui.add(
                    egui::Slider::new(&mut system.terrain_config.visible_chunk_radius, 1..=32)
                        .text("Visible Chunk Radius"),
                );
                // loaded chunks
                ui.label(format!("Loaded Chunks: {}", system.terrain.chunks.len()));
            });

        player_camera.speed = camera_speed;
        player_camera.sensitivity = camera_sensitivity;
        player_camera.set_fov(camera_fov);

        system.globals.enable_lighting = lighting as u32;
    }

    ok()
}
