use vek::Vec3;

use crate::render::Vertex;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct TerrainVertex {
    pub positions: [f32; 3],
    pub texture_id: u32,
    pub normal: [i32; 3],
}

impl TerrainVertex {
    pub fn new(position: vek::Vec3<i32>, texture_id: u32, normal: Vec3<i32>) -> Self {
        // TODO: pack vertex data
        Self {
            positions: position.map(|f| f as f32).into_array(),
            texture_id,
            normal: normal.into_array(),
        }
    }
}

impl Vertex for TerrainVertex {
    const INDEX_BUFFER: Option<wgpu::IndexFormat> = Some(wgpu::IndexFormat::Uint32);

    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        const ATTRS: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
            0 => Float32x3,
            1 => Uint32,
            2 => Sint32x3,
        ];
        wgpu::VertexBufferLayout {
            array_stride: Self::STRIDE,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRS,
        }
    }
}
