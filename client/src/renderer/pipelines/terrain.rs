use vek::{
    num_traits::{One, Zero},
    Vec3,
};

use crate::{
    png_utils,
    renderer::{buffer::Buffer, texture::Texture},
};

const SHADER: &str = include_str!("../../../../assets/shaders/terrain.wgsl");

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TerrainVertex {
    pub position: [f32; 3],
    pub texture_coords: [f32; 2],
}

impl TerrainVertex {
    pub fn new(pos: Vec3<f32>, texture_coords: [f32; 2]) -> Self {
        Self {
            position: pos.into_array(),
            texture_coords,
        }
    }
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        const ATTRS: [wgpu::VertexAttribute; 2] =
            wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];
        wgpu::VertexBufferLayout {
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRS,
            array_stride: std::mem::size_of::<TerrainVertex>() as wgpu::BufferAddress,
        }
    }
}

pub struct TerrainPipeline {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: Buffer<TerrainVertex>,
    index_buffer: Buffer<u32>,
    material_bind_group: wgpu::BindGroup,
}

impl TerrainPipeline {
    pub fn new(
        device: &wgpu::Device,
        surface_config: &wgpu::SurfaceConfiguration,
        queue: &wgpu::Queue,
        common_bind_groups: &[&wgpu::BindGroupLayout],
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });

        let image = png_utils::read("assets/textures/blocks/grass_side.png").unwrap();
        let texture = Texture::new(device, queue, image);

        let material_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Material Bind Group Layout"),
                entries: &[
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

        let material_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Material Bind Group"),
            layout: &material_bind_group_layout,
            entries: &[
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

        // create layouts with common bind groups + material bind group
        let layouts = common_bind_groups
            .iter()
            .copied()
            .chain(std::iter::once(&material_bind_group_layout))
            .collect::<Vec<_>>();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &layouts,
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[TerrainVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::all(),
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            // depth_stencil: Some(wgpu::DepthStencilState {
            //     format: texture::Texture::DEPTH_FORMAT,
            //     depth_write_enabled: true,
            //     depth_compare: wgpu::CompareFunction::Less,
            //     stencil: wgpu::StencilState::default(),
            //     bias: wgpu::DepthBiasState::default(),
            // }),
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let (vertex_buffer, index_buffer) = create_cube(device);

        Self {
            pipeline: render_pipeline,
            vertex_buffer,
            index_buffer,
            material_bind_group,
        }
    }

    pub fn draw<'a>(&'a mut self, pass: &mut wgpu::RenderPass<'a>, common_bg: &'a wgpu::BindGroup) {
        pass.set_bind_group(0, common_bg, &[]);
        pass.set_bind_group(1, &self.material_bind_group, &[]);
        pass.set_pipeline(&self.pipeline);
        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.draw_indexed(0..self.index_buffer.len(), 0, 0..1);
    }
}

fn create_cube(device: &wgpu::Device) -> (Buffer<TerrainVertex>, Buffer<u32>) {
    let origin = Vec3::zero();
    let vertices = vec![
        // North
        TerrainVertex::new(origin + Vec3::unit_x() + Vec3::unit_z(), [0.0, 0.0]),
        TerrainVertex::new(origin + Vec3::unit_z(), [0.0, 1.0]),
        TerrainVertex::new(origin + Vec3::unit_z() + Vec3::unit_y(), [1.0, 1.0]),
        TerrainVertex::new(
            origin + Vec3::unit_z() + Vec3::unit_x() + Vec3::unit_y(),
            [1.0, 0.0],
        ),
        // South
        TerrainVertex::new(origin, [0.0, 0.0]),
        TerrainVertex::new(origin + Vec3::unit_x(), [0.0, 1.0]),
        TerrainVertex::new(origin + Vec3::unit_x() + Vec3::unit_y(), [1.0, 1.0]),
        TerrainVertex::new(origin + Vec3::unit_y(), [1.0, 0.0]),
        // East
        TerrainVertex::new(origin + Vec3::unit_x(), [0.0, 0.0]),
        TerrainVertex::new(origin + Vec3::unit_x() + Vec3::unit_z(), [0.0, 1.0]),
        TerrainVertex::new(
            origin + Vec3::unit_x() + Vec3::unit_z() + Vec3::unit_y(),
            [1.0, 1.0],
        ),
        TerrainVertex::new(origin + Vec3::unit_x() + Vec3::unit_y(), [1.0, 0.0]),
        // West
        TerrainVertex::new(origin + Vec3::unit_z(), [0.0, 0.0]),
        TerrainVertex::new(origin, [0.0, 1.0]),
        TerrainVertex::new(origin + Vec3::unit_y(), [1.0, 1.0]),
        TerrainVertex::new(origin + Vec3::unit_z() + Vec3::unit_y(), [1.0, 0.0]),
        // Bottom
        TerrainVertex::new(origin, [0.0, 0.0]),
        TerrainVertex::new(origin + Vec3::unit_z(), [0.0, 1.0]),
        TerrainVertex::new(origin + Vec3::unit_x() + Vec3::unit_z(), [1.0, 1.0]),
        TerrainVertex::new(origin + Vec3::unit_x(), [1.0, 0.0]),
        // Top
        TerrainVertex::new(origin + Vec3::unit_y(), [0.0, 0.0]),
        TerrainVertex::new(origin + Vec3::unit_y() + Vec3::unit_x(), [0.0, 1.0]),
        TerrainVertex::new(
            origin + Vec3::unit_y() + Vec3::unit_x() + Vec3::unit_z(),
            [1.0, 1.0],
        ),
        TerrainVertex::new(origin + Vec3::unit_y() + Vec3::unit_z(), [1.0, 0.0]),
    ];

    let vertex_buffer = Buffer::new(device, wgpu::BufferUsages::VERTEX, &vertices);
    (
        vertex_buffer,
        compute_terrain_indices(device, vertices.len()),
    )
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
