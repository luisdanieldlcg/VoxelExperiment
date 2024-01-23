pub mod terrain;

pub struct Pipelines {
    pub terrain: terrain::TerrainPipeline,
}

impl Pipelines {
    pub fn new(
        device: &wgpu::Device,
        surface_config: &wgpu::SurfaceConfiguration,
        common_bind_groups: &[&wgpu::BindGroupLayout],
    ) -> Self {
        Self {
            terrain: terrain::TerrainPipeline::new(device, surface_config, common_bind_groups),
        }
    }
}
