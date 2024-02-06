use std::borrow::Cow;

use self::terrain::TerrainPipeline;

use super::texture::Texture;

pub mod terrain;

const VERT_SHADER_ENTRY_POINT: &str = "vs_main";
const FRAG_SHADER_ENTRY_POINT: &str = "fs_main";

pub struct CreatePipelineInfo<'a> {
    pub(super) device: &'a wgpu::Device,
    pub(super) surface_config: &'a wgpu::SurfaceConfiguration,
    pub(super) common_bg: &'a wgpu::BindGroupLayout,
    pub(super) atlas: Texture,
}

pub struct PipelineLayouts {}

pub struct Shaders {
    terrain: wgpu::ShaderModule,
}

impl Shaders {
    pub fn new(device: &wgpu::Device) -> Self {
        log::info!("Creating ShaderModules...");
        let create_shader = |filename| {
            let full_specifier = ["assets/shaders/", filename, ".wgsl"].concat();
            log::debug!("Loading shader: {}", full_specifier);
            let source = std::fs::read_to_string(&full_specifier)
                .unwrap_or_else(|_| panic!("Failed to read shader file: {}", full_specifier));

            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(filename),
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(&source)),
            })
        };

        Self {
            terrain: create_shader("terrain"),
        }
    }

    pub fn create_pipelines(&self, info: CreatePipelineInfo) -> Pipelines {
        Pipelines {
            terrain: TerrainPipeline::new(
                &self.terrain,
                info.device,
                info.surface_config,
                &[info.common_bg],
                info.atlas,
            ),
        }
    }
}

pub struct Pipelines {
    pub terrain: terrain::TerrainPipeline,
}
