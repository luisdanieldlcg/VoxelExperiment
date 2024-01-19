use vek::Vec3;

use crate::render::Vertex;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct TerrainVertex {
    pub data: u32,
    pub normal: [i32; 3],
}

impl TerrainVertex {
    pub fn new(position: vek::Vec3<u32>, texture_id: u16, normal: Vec3<i32>) -> Self {
        Self {
            data: (position.x << 27) | (position.y << 18) | (position.z << 13) | (texture_id as u32),
            normal: [normal.x, normal.y, normal.z],
        }
    }
}

impl Vertex for TerrainVertex {
    const INDEX_BUFFER: Option<wgpu::IndexFormat> = Some(wgpu::IndexFormat::Uint32);

    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        const ATTRS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
            0 => Uint32,
            1 => Sint32x3,
        ];
        wgpu::VertexBufferLayout {
            array_stride: Self::STRIDE,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRS,
        }
    }
}
