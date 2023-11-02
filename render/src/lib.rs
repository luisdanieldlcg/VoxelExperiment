pub mod atlas;
pub mod buffer;
pub mod error;
pub mod mesh;
pub mod pipeline;
pub mod texture;
pub mod vertex;

use buffer::Buffer;
use common::state::SysResult;
use log::info;
use texture::Texture;
use vek::Mat4;
use vertex::TerrainVertex;

#[derive(Default)]
pub struct TerrainRenderData {
    pub buffer: Option<Buffer<TerrainVertex>>,
    pub wireframe_enabled: bool,
}

#[derive(Default)]
pub struct Blocks {
    pub map: atlas::BlockMap,
}
pub trait Vertex: bytemuck::Pod {
    const STRIDE: wgpu::BufferAddress = std::mem::size_of::<Self>() as wgpu::BufferAddress;

    const INDEX_BUFFER: Option<wgpu::IndexFormat>;
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct GpuGlobals {
    pub view: [[f32; 4]; 4],
    pub proj: [[f32; 4]; 4],
}

impl GpuGlobals {
    pub fn new(view: Mat4<f32>, proj: Mat4<f32>) -> Self {
        Self {
            view: view.into_col_arrays(),
            proj: proj.into_col_arrays(),
        }
    }
}
impl Default for GpuGlobals {
    fn default() -> Self {
        Self::new(Mat4::identity(), Mat4::identity())
    }
}

pub struct Pipelines {
    pub terrain: pipeline::TerrainPipeline,
    pub terrain_wireframe: pipeline::TerrainPipeline,
}

pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pipelines: Pipelines,
    globals_buffer: Buffer<GpuGlobals>,
    terrain_index_buffer: Buffer<u32>,
    globals_bind_group: wgpu::BindGroup,
    texture_bind_group: wgpu::BindGroup,
    depth_texture: Texture,
    block_map: atlas::BlockMap,
}

impl Renderer {
    pub fn new(window: &winit::window::Window) -> Result<Self, error::RenderError> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            flags: wgpu::InstanceFlags::default(),
            dx12_shader_compiler: wgpu::Dx12Compiler::default(),
            gles_minor_version: wgpu::Gles3MinorVersion::default(),
        });

        let surface = unsafe { instance.create_surface(window) }.unwrap();
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .ok_or(error::RenderError::AdapterNotFound)?;

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::POLYGON_MODE_LINE,
                limits: wgpu::Limits::default(),
                label: None,
            },
            None, // Trace path
        ))?;

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let size = window.inner_size();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: Vec::with_capacity(0),
        };
        surface.configure(&device, &config);

        let shader =
            device.create_shader_module(wgpu::include_wgsl!("../../assets/shaders/terrain.wgsl"));

        let globals_buffer = Buffer::new(
            &device,
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            &[GpuGlobals::default()],
        );

        let globals_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Globals Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let terrain_index_buffer = compute_terrain_indices(&device, 5000);

        let globals_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Globals Bind Group"),
            layout: &globals_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: globals_buffer.as_entire_binding(),
            }],
        });
        let (atlas_buffer, atlas_tiles) = atlas::create_atlas("assets/textures/block", 16, 16);
        let blocks = atlas::load_blocks("assets/blocks", atlas_tiles);

        let atlas_texture = texture::Texture::new(
            &device,
            &queue,
            image::DynamicImage::ImageRgba8(atlas_buffer),
        );

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Texture Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Texture Bind Group"),
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&atlas_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&atlas_texture.sampler),
                },
            ],
        });
        let pipelines = Pipelines {
            terrain: pipeline::TerrainPipeline::new(
                &device,
                &[&globals_bind_group_layout, &texture_bind_group_layout],
                &shader,
                &config,
                false,
            ),
            terrain_wireframe: pipeline::TerrainPipeline::new(
                &device,
                &[&globals_bind_group_layout, &texture_bind_group_layout],
                &shader,
                &config,
                true,
            ),
        };
        let depth_texture = Texture::depth(&device, config.width, config.height);
        Ok(Self {
            surface,
            device,
            queue,
            config,
            terrain_index_buffer,
            globals_buffer,
            globals_bind_group: globals_bindgroup,
            texture_bind_group,
            pipelines,
            depth_texture,
            block_map: blocks,
        })
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        if new_width == 0 || new_height == 0 {
            // Resize with 0 width and height is used by winit to signal a minimize event on Windows.
            // Refer to: https://github.com/rust-windowing/winit/issues/208
            // This solves an issue where the app would panic when minimizing on Windows.
            return;
        }
        self.config.width = new_width;
        self.config.height = new_height;
        self.depth_texture = Texture::depth(&self.device, new_width, new_height);
        self.surface.configure(&self.device, &self.config);
    }

    pub fn write_globals(&mut self, globals: GpuGlobals) {
        self.globals_buffer.write(&self.queue, &[globals]);
    }

    pub fn create_vertex_buffer<T: Vertex>(&mut self, data: &[T]) -> Buffer<T> {
        self.check_index_buffer::<T>(data.len());
        Buffer::new(&self.device, wgpu::BufferUsages::VERTEX, data)
    }

    pub fn set_block_resource(&self, ecs: &mut apecs::World) {
        info!("Blocks in block map: {}", self.block_map.len());
        let blocks = ecs.resource_mut::<Blocks>().unwrap();
        *blocks = Blocks {
            map: self.block_map.clone(),
        };
    }

    pub fn check_index_buffer<V: Vertex>(&mut self, len: usize) {
        let l = len / 6 * 4;
        match V::INDEX_BUFFER {
            Some(wgpu::IndexFormat::Uint16) => {
                // TODO: create u16 index buffer
            },
            Some(wgpu::IndexFormat::Uint32) => {
                if self.terrain_index_buffer.len() > l as u32 {
                    return;
                }
                if len > u32::MAX as usize {
                    panic!(
                        "Too many vertices for {} using u32 index buffer. Count: {}",
                        core::any::type_name::<V>(),
                        len
                    );
                }
                log::info!(
                    "Recreating index buffer for {}, with {} vertices",
                    core::any::type_name::<V>(),
                    len
                );
                self.terrain_index_buffer = compute_terrain_indices(&self.device, len);
            },

            None => (),
        }
    }
}

use apecs::*;
pub fn render_system(
    (renderer, terrain_render_data): (
        Read<Renderer, NoDefault>,
        Read<TerrainRenderData, NoDefault>,
    ),
) -> SysResult {
    let output = renderer.surface.get_current_texture()?;
    let view = output
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = renderer
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0,
                }),
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: &renderer.depth_texture.view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        }),
        occlusion_query_set: None,
        timestamp_writes: None,
    });
    if terrain_render_data.wireframe_enabled {
        render_pass.set_pipeline(&renderer.pipelines.terrain_wireframe.pipeline);
    } else {
        render_pass.set_pipeline(&renderer.pipelines.terrain.pipeline);
    }
    render_pass.set_bind_group(0, &renderer.globals_bind_group, &[]);
    render_pass.set_bind_group(1, &renderer.texture_bind_group, &[]);
    if let Some(buffer) = &terrain_render_data.buffer {
        render_pass.set_vertex_buffer(0, buffer.slice());
    }
    render_pass.set_index_buffer(
        renderer.terrain_index_buffer.slice(),
        wgpu::IndexFormat::Uint32,
    );
    render_pass.draw_indexed(0..renderer.terrain_index_buffer.len(), 0, 0..1);

    drop(render_pass);
    renderer.queue.submit(Some(encoder.finish()));
    output.present();
    Ok(ShouldContinue::Yes)
}

fn compute_terrain_indices(device: &wgpu::Device, vert_length: usize) -> Buffer<u32> {
    assert!(vert_length <= u32::MAX as usize);
    let indices = [0, 1, 2, 2, 3, 0]
        .iter()
        .cycle()
        .copied()
        .take(vert_length / 4 * 6)
        .enumerate()
        .map(|(i, b)| (i / 6 * 4 + b) as u32)
        .collect::<Vec<_>>();

    Buffer::new(device, wgpu::BufferUsages::INDEX, &indices)
}
