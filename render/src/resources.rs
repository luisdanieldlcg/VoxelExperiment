use crate::{buffer::Buffer, vertex::TerrainVertex};

#[derive(Default)]
pub struct TerrainRenderData {
    pub buffer: Option<Buffer<TerrainVertex>>,
    pub wireframe: bool,
    pub ready: bool,
}

#[derive(Debug, Clone, Default)]
pub struct EguiContext(egui::Context);

impl EguiContext {
    pub fn get(&self) -> &egui::Context {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut egui::Context {
        &mut self.0
    }
}

#[derive(Debug, Clone, Default)]
pub struct EguiSettings {
    pub scale_factor: f32,
}
