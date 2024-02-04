use crate::camera::Matrices;

use self::{buffer::Buffer, pipelines::Pipelines, texture_packer::Atlas};
use std::sync::Arc;
use vek::Mat4;
use winit::window::Window;

pub mod buffer;
pub mod pipelines;
pub mod texture;
pub mod texture_packer;

/// Manages the rendering of the application.
pub struct Renderer {
    /// The window used for rendering.
    window: Arc<Window>,
    /// Surface on which the renderer will draw.
    surface: wgpu::Surface<'static>,
    /// The GPU device. Contains functions used for interacting with the GPU.
    device: wgpu::Device,
    /// A GPU Queue handle. Used for submitting commands to the GPU.
    queue: wgpu::Queue,
    /// The surface configuration. This is used for configuring the surface.
    config: wgpu::SurfaceConfiguration,
    /// The render pipeline configuration.
    pipelines: Pipelines,
    /// The common bind group. This is used for storing data that is common to all shaders.
    common_bind_group: wgpu::BindGroup,
    /// Global uniforms
    uniforms_buffer: Buffer<Uniforms>,
    /// Block Texture Atlas
    atlas: Atlas,
}

impl Renderer {
    pub fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // The surface needs to live as long as the window that created it.
        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
            },
            None, // Trace path
        ))
        .unwrap();
        let (width, height) = window.inner_size().into();
        let config = surface.get_default_config(&adapter, width, height).unwrap();
        surface.configure(&device, &config);
        
        let atlas = texture_packer::Atlas::pack_textures("assets/textures/blocks");

        let uniforms = Uniforms::default();
        let uniforms_buffer = Buffer::new(
            &device,
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            &[uniforms],
        );

        let common_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Common Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let common_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Common Bind Group"),
            layout: &common_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniforms_buffer.as_entire_binding(),
            }],
        });
        
        let pipelines = Pipelines::new(
            &device,
            &queue,
            &config,
            &[&common_bind_group_layout],
            &atlas,
        );
        log::info!("Renderer initialized.");

        Self {
            surface,
            device,
            queue,
            config,
            window,
            pipelines,
            common_bind_group,
            uniforms_buffer,
            atlas,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn write_uniforms(&mut self, uniforms: Uniforms) {
        self.uniforms_buffer.write(&self.queue, &[uniforms]);
    }

    pub fn render(&mut self, matrices: Matrices) {
        self.write_uniforms(Uniforms::new(
            matrices.view,
            matrices.proj,
            self.atlas.image.width,
            self.atlas.tile_size,
        ));

        let output = match self.surface.get_current_texture() {
            Ok(t) => t,
            Err(wgpu::SurfaceError::Lost) => {
                let (width, height) = self.window.inner_size().into();
                self.resize(width, height);
                log::warn!("Surface lost, resizing. A frame will be dropped.");
                return;
            },
            Err(e) => panic!("{:#?}", e),
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
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
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            self.pipelines
                .terrain
                .draw(&mut render_pass, &self.common_bind_group);
        }
        self.queue.submit(Some(encoder.finish()));
        output.present();
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    view: [[f32; 4]; 4],
    proj: [[f32; 4]; 4],
    atlas_size: u32,
    atlas_tile_size: u32,
    _pad: [u32; 2],
}

const _: () = assert!(core::mem::size_of::<Uniforms>() % 16 == 0);

impl Default for Uniforms {
    fn default() -> Self {
        Self {
            view: Mat4::identity().into_col_arrays(),
            proj: Mat4::identity().into_col_arrays(),
            atlas_size: 0,
            atlas_tile_size: 0,
            _pad: [0; 2],
        }
    }
}

impl Uniforms {
    pub fn new(view: Mat4<f32>, proj: Mat4<f32>, atlas_size: u32, atlas_tile_size: u32) -> Self {
        Self {
            view: view.into_col_arrays(),
            proj: proj.into_col_arrays(),
            atlas_size: 0,
            atlas_tile_size: 0,
            _pad: [0; 2],
        }
    }
}
