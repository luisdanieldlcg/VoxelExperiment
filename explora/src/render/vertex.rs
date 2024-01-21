use vek::Vec3;

use crate::render::Vertex;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct TerrainVertex {
    pub data: u32,
}

impl TerrainVertex {
    pub fn new(position: vek::Vec3<u32>, texture_id: u16, normal: Vec3<i32>) -> Self {
        // pack normals
        // since normals are in the range [-1, 1], we can map it to [0, 1] by adding 1 and dividing by 2
        let normal = normal.map(|x| (x + 1) / 2).map(|x| x as u8);
        Self {
            data: (position.x << 27)
                | (position.y << 18)
                | (position.z << 13)
                // pack normals. each one is 1 bits, where 1 means positive and 0 means negative
                | ((normal.x as u32) << 12)
                | ((normal.y as u32) << 11)
                | ((normal.z as u32) << 10)
                | (texture_id as u32),
        }
    }
}

impl Vertex for TerrainVertex {
    const INDEX_BUFFER: Option<wgpu::IndexFormat> = Some(wgpu::IndexFormat::Uint32);

    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        const ATTRS: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![
            0 => Uint32,
        ];
        wgpu::VertexBufferLayout {
            array_stride: Self::STRIDE,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRS,
        }
    }
}
