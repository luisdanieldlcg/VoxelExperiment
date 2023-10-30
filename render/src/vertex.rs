use crate::Vertex;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct TerrainVertex {
    pub positions: [f32; 3],
    pub texture_id: u32,
}

impl TerrainVertex {
    pub fn new(positions: vek::Vec3<i32>, texture_id: u32) -> Self {
        Self {
            positions: positions.map(|f| f as f32).into_array(),
            texture_id,
        }
    }
}

impl Vertex for TerrainVertex {
    const INDEX_BUFFER: Option<wgpu::IndexFormat> = Some(wgpu::IndexFormat::Uint32);

    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        const ATTRS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
            0 => Float32x3,
            1 => Uint32,
        ];
        wgpu::VertexBufferLayout {
            array_stride: Self::STRIDE,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRS,
        }
    }
}
