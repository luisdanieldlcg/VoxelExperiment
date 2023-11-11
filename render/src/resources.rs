use std::collections::HashMap;

use vek::Vec2;

use crate::{buffer::Buffer, vertex::TerrainVertex};

#[derive(Default)]
pub struct TerrainRender {
    pub chunks: HashMap<Vec2<i32>, TerrainRenderData>,
    pub wireframe: bool,
}

pub struct TerrainRenderData {
    pub buffer: Buffer<TerrainVertex>,
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
