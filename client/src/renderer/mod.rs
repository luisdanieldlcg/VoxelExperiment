use crate::{png_utils, renderer::texture::Texture};

use self::pipelines::Pipelines;
use std::sync::Arc;
use winit::window::Window;

pub mod pipelines;
pub mod texture;
pub mod texture_packer;

/// Manages the rendering of the application.
pub struct Renderer {
    window: Arc<Window>,
    /// Surface on which the renderer will draw.
    surface: wgpu::Surface<'static>,
    /// The GPU device. Contains functions used for interacting with the GPU.\
    device: wgpu::Device,
    /// A GPU Queue handle. Used for submitting commands to the GPU.
    queue: wgpu::Queue,
    /// The surface configuration. This is used for configuring the surface.
    config: wgpu::SurfaceConfiguration,
    /// The render pipeline configuration.
    pipelines: Pipelines,
    common_bind_group: wgpu::BindGroup,
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

        let image = png_utils::read("assets/textures/blocks/grass_side.png").unwrap();
        let texture = Texture::new(&device, &queue, image);

        let common_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Common Bind Group Layout"),
                entries: &[
                    // // Globals
                    // wgpu::BindGroupLayoutEntry {
                    //     binding: 0,
                    //     visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    //     ty: wgpu::BindingType::Buffer {
                    //         ty: wgpu::BufferBindingType::Uniform,
                    //         has_dynamic_offset: false,
                    //         min_binding_size: None,
                    //     },
                    //     count: None,
                    // },
                    // Atlas Texture
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
                    // Atlas Texture Sampler
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let common_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Common Bind Group"),
            layout: &common_bind_group_layout,
            entries: &[
                // wgpu::BindGroupEntry {
                //     binding: 0,
                //     resource: uniforms_buffer.as_entire_binding(),
                // },
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
        });

        let pipelines = Pipelines::new(&device, &config, &[&common_bind_group_layout]);
        texture_packer::pack_textures("assets/textures/blocks");
        tracing::info!("Renderer initialized.");
        Self {
            surface,
            device,
            queue,
            config,
            window,
            pipelines,
            common_bind_group,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(&mut self) {
        let output = match self.surface.get_current_texture() {
            Ok(t) => t,
            Err(wgpu::SurfaceError::Lost) => {
                let (width, height) = self.window.inner_size().into();
                self.resize(width, height);
                tracing::warn!("Surface lost, resizing. A frame will be dropped.");
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
