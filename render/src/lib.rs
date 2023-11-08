pub mod atlas;
pub mod bindings;
pub mod buffer;
pub mod error;
pub mod pipeline;
pub mod resources;
pub mod texture;
pub mod ui;
pub mod vertex;

use atlas::BlockAtlas;
use buffer::Buffer;
use resources::{EguiContext, TerrainRenderData};
use texture::Texture;
use vek::{Mat4, Vec3};

pub const SYSTEM_STAGE_PRE_RENDER: &str = "pre_render";
pub const SYSTEM_STAGE_RENDER: &str = "render";
pub const SYSTEM_STAGE_UI_DRAW_WIDGETS: &str = "ui_draw_widgets";
pub const SYSTEM_STAGE_UI_RENDER: &str = "ui_render";
pub const SYSTEM_STAGE_POST_RENDER: &str = "post_render";
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
    pub sun_pos: [f32; 3],
    pub enable_lighting: u32,
}

impl GpuGlobals {
    pub fn new(view: Mat4<f32>, proj: Mat4<f32>, sun_pos: Vec3<f32>, lighting: u32) -> Self {
        Self {
            view: view.into_col_arrays(),
            proj: proj.into_col_arrays(),
            sun_pos: sun_pos.into_array(),
            enable_lighting: lighting,
        }
    }
}
impl Default for GpuGlobals {
    fn default() -> Self {
        Self::new(Mat4::identity(), Mat4::identity(), Vec3::zero(), 1)
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
    core_bind_group: wgpu::BindGroup,
    depth_texture: Texture,
    block_atlas: BlockAtlas,
    egui_renderer: egui_wgpu::Renderer,
    // For debugging
    pub graphics_backend: String,
}

impl Renderer {
    pub fn initialize(window: &winit::window::Window) -> Result<apecs::Plugin, error::RenderError> {
        let backends = std::env::var("WGPU_BACKEND")
            .ok()
            .and_then(|env| match env.to_lowercase().as_str() {
                "vulkan" => Some(wgpu::Backends::VULKAN),
                "metal" => Some(wgpu::Backends::METAL),
                "dx12" => Some(wgpu::Backends::DX12),
                "dx11" => Some(wgpu::Backends::DX11),
                "opengl" => Some(wgpu::Backends::GL),
                "primary" => Some(wgpu::Backends::PRIMARY),
                "secondary" => Some(wgpu::Backends::SECONDARY),
                "all" => Some(wgpu::Backends::all()),
                _ => None,
            })
            .unwrap_or(wgpu::Backends::PRIMARY);

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends,
            // flags: wgpu::InstanceFlags::default(),
            dx12_shader_compiler: wgpu::Dx12Compiler::default(),
            // gles_minor_version: wgpu::Gles3MinorVersion::default(),
        });

        let surface = unsafe { instance.create_surface(window) }.unwrap();
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .ok_or(error::RenderError::AdapterNotFound)?;

        let adapter_info = adapter.get_info();

        log::info!(
            "Selected graphics device: {} {} {:?} {:?}",
            adapter_info.name,
            adapter_info.vendor,
            adapter_info.backend,
            adapter_info.device_type
        );

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

        let block_atlas = match BlockAtlas::create(&device, &queue, "assets/textures/block", 16, 16)
        {
            Ok(atlas) => atlas,
            Err(err) => {
                panic!("Failed to create block atlas: {}", err);
                // TODO: return custom error? (e.g RendererError::BlockAtlasCreationFailed)
            },
        };

        let core_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Core Bind Group Layout"),
                entries: &bindings::core_bind_group_layouts(),
            });

        let core_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Core Bind Group"),
            layout: &core_bind_group_layout,
            entries: &bindings::core_bind_groups(&globals_buffer, &block_atlas),
        });

        let pipelines = Pipelines {
            terrain: pipeline::TerrainPipeline::new(
                &device,
                &[&core_bind_group_layout],
                &shader,
                &config,
                false,
            ),
            terrain_wireframe: pipeline::TerrainPipeline::new(
                &device,
                &[&core_bind_group_layout],
                &shader,
                &config,
                true,
            ),
        };
        let depth_texture = Texture::depth(&device, config.width, config.height);
        let terrain_index_buffer = compute_terrain_indices(&device, 5000);
        let egui_renderer = egui_wgpu::Renderer::new(&device, surface_format, None, 1);
        let graphics_backend = format!("{:?}", adapter_info.backend);

        let this = Self {
            surface,
            device,
            queue,
            config,
            terrain_index_buffer,
            globals_buffer,
            core_bind_group,
            pipelines,
            depth_texture,
            block_atlas,
            egui_renderer,
            graphics_backend,
        };

        Ok(Self::setup_ecs_plugin(this))
    }

    fn setup_ecs_plugin(self) -> apecs::Plugin {
        apecs::Plugin::default()
            .with_resource(|_: ()| Ok(self))
            .with_resource(|_: ()| Ok(GpuGlobals::default()))
            .with_resource(|_: ()| Ok(TerrainRenderData::default()))
            .with_resource(|_: ()| Ok(EguiContext::default()))
            .with_system(
                SYSTEM_STAGE_PRE_RENDER,
                pre_render_system,
                &[SYSTEM_STAGE_RENDER],
                &[],
            )
            .with_system(
                SYSTEM_STAGE_RENDER,
                render_system,
                &[SYSTEM_STAGE_UI_DRAW_WIDGETS],
                &[SYSTEM_STAGE_PRE_RENDER],
            )
            .with_system(
                SYSTEM_STAGE_UI_RENDER,
                ui::ui_render_system,
                &[],
                &[SYSTEM_STAGE_UI_DRAW_WIDGETS],
            )
            .with_system(
                SYSTEM_STAGE_POST_RENDER,
                post_render_system,
                &[],
                &[SYSTEM_STAGE_UI_RENDER],
            )
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

    pub fn block_atlas(&self) -> &atlas::BlockAtlas {
        &self.block_atlas
    }

    pub fn update_ui_texture(
        &mut self,
        id: egui::TextureId,
        image_delta: &egui::epaint::ImageDelta,
    ) {
        self.egui_renderer
            .update_texture(&self.device, &self.queue, id, image_delta);
    }

    pub fn update_ui_buffers(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        paint_jobs: &[egui::epaint::ClippedPrimitive],
        screen_descriptor: &egui_wgpu::renderer::ScreenDescriptor,
    ) {
        self.egui_renderer.update_buffers(
            &self.device,
            &self.queue,
            encoder,
            paint_jobs,
            screen_descriptor,
        );
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

struct RenderTexture {
    surface_tex: wgpu::SurfaceTexture,
    surface_tex_view: wgpu::TextureView,
}

struct CommandEncoder {
    encoder: wgpu::CommandEncoder,
}

#[derive(CanFetch)]
struct PreRenderSystem {
    encoder: Write<Option<CommandEncoder>>,
    texture: Write<Option<RenderTexture>>,
    renderer: Read<Renderer, NoDefault>,
}

fn pre_render_system(mut system: PreRenderSystem) -> apecs::anyhow::Result<ShouldContinue> {
    let renderer = system.renderer;
    let surface = match renderer.surface.get_current_texture() {
        Ok(t) => t,
        Err(err) => {
            match err {
                wgpu::SurfaceError::Timeout | wgpu::SurfaceError::Outdated => {
                    log::warn!("{:?}", err);
                    return ok();
                },
                wgpu::SurfaceError::Lost => {
                    log::warn!("Swapchain is lost, recreating...");
                    renderer
                        .surface
                        .configure(&renderer.device, &renderer.config);
                    return ok();
                },
                wgpu::SurfaceError::OutOfMemory => {
                    panic!("Render system error: There is no more memory left to allocate a new frame. ");
                },
            }
        },
    };
    let view = surface
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let encoder = renderer
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

    let texture = RenderTexture {
        surface_tex: surface,
        surface_tex_view: view,
    };

    let encoder = CommandEncoder { encoder };
    // update options
    system.encoder.replace(encoder);
    system.texture.replace(texture);
    ok()
}

#[derive(CanFetch)]
struct RenderSystem {
    renderer: Read<Renderer, NoDefault>,
    terrain_render_data: Read<TerrainRenderData, NoDefault>,
    texture: Write<Option<RenderTexture>>,
    encoder: Write<Option<CommandEncoder>>,
}

/// Sets up the main render pass and draws the terrain
fn render_system(mut system: RenderSystem) -> apecs::anyhow::Result<ShouldContinue> {
    let renderer = &system.renderer;
    // borrow inner option T mutably
    let texture = system.texture.inner_mut().as_mut().unwrap();
    let encoder = &mut system.encoder.inner_mut().as_mut().unwrap().encoder;

    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &texture.surface_tex_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0,
                }),
                // store: wgpu::StoreOp::Store,
                store: true,
            },
        })],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: &system.renderer.depth_texture.view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                // store: wgpu::StoreOp::Store,
                store: true,
            }),
            stencil_ops: None,
        }),
        // occlusion_query_set: None,
        // timestamp_writes: None,
    });
    if system.terrain_render_data.wireframe {
        render_pass.set_pipeline(&renderer.pipelines.terrain_wireframe.pipeline);
    } else {
        render_pass.set_pipeline(&renderer.pipelines.terrain.pipeline);
    }
    render_pass.set_bind_group(0, &renderer.core_bind_group, &[]);
    if let Some(buffer) = &system.terrain_render_data.buffer {
        render_pass.set_vertex_buffer(0, buffer.slice());
    }
    render_pass.set_index_buffer(
        renderer.terrain_index_buffer.slice(),
        wgpu::IndexFormat::Uint32,
    );
    render_pass.draw_indexed(0..renderer.terrain_index_buffer.len(), 0, 0..1);
    ok()
}

#[derive(CanFetch)]
struct PostRenderSystem {
    texture: Write<Option<RenderTexture>>,
    command_encoder: Write<Option<CommandEncoder>>,
    renderer: Read<Renderer, NoDefault>,
}

fn post_render_system(mut system: PostRenderSystem) -> apecs::anyhow::Result<ShouldContinue> {
    let texture = system.texture.inner_mut();
    let command_encoder = system.command_encoder.inner_mut();
    let texture = texture.take();
    let command_encoder = command_encoder.take();

    if let (Some(texture), Some(command_encoder)) = (texture, command_encoder) {
        let texture = texture.surface_tex;
        let command_encoder = command_encoder.encoder;
        system.renderer.queue.submit(Some(command_encoder.finish()));
        texture.present();
    }
    ok()
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
