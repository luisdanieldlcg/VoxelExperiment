use std::collections::HashMap;

use vek::Vec2;

use crate::render::{buffer::Buffer, vertex::TerrainVertex};

use super::ChunkPos;

#[derive(Default)]
pub struct TerrainRender {
    pub chunks: HashMap<Vec2<i32>, TerrainChunkMesh>,
    pub wireframe: bool,
}

pub struct TerrainChunkMesh {
    pub vertex_buffer: Buffer<TerrainVertex>,
    pub chunk_pos_buffer: Buffer<ChunkPos>,
    pub chunk_pos_bind_group: wgpu::BindGroup,
}

impl TerrainChunkMesh {
    pub fn new(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        chunk_pos: ChunkPos,
        vertex_buffer: Buffer<TerrainVertex>,
    ) -> Self {
        let chunk_pos_buffer = Buffer::new(
            device,
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            &[chunk_pos],
        );

        let chunk_pos_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Chunk Pos Bind Group"),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: chunk_pos_buffer.as_entire_binding(),
            }],
        });

        Self {
            vertex_buffer,
            chunk_pos_buffer,
            chunk_pos_bind_group,
        }
    }
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
