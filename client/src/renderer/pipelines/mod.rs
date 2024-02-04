use super::texture_packer;

pub mod terrain;

pub struct Pipelines {
    pub terrain: terrain::TerrainPipeline,
}

impl Pipelines {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        surface_config: &wgpu::SurfaceConfiguration,
        common_bind_groups: &[&wgpu::BindGroupLayout],
        atlas: &texture_packer::Atlas,
    ) -> Self {
        Self {
            terrain: terrain::TerrainPipeline::new(
                device,
                surface_config,
                queue,
                common_bind_groups,
                atlas,
            ),
        }
    }
}
