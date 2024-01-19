use apecs::{anyhow::Result, *};

use crate::render::{
    resources::{EguiContext, EguiSettings},
    CommandEncoder, RenderTexture, Renderer,
};

#[derive(CanFetch)]
pub struct UiRenderSystem {
    encoder: Write<Option<CommandEncoder>>,
    texture: Write<Option<RenderTexture>>,
    renderer: Write<Renderer, NoDefault>,
    egui_context: Write<EguiContext>,
    egui_configuration: Read<EguiSettings>,
}

pub fn ui_render_system(mut ui: UiRenderSystem) -> Result<ShouldContinue> {
    let encoder = &mut ui.encoder.inner_mut().as_mut().unwrap().encoder;
    let texture = &mut ui.texture.inner_mut().as_mut().unwrap();

    let egui_context = ui.egui_context.inner_mut();
    let output = egui_context.get_mut().end_frame();
    let paint_jobs = egui_context
        .get_mut()
        .tessellate(output.shapes, output.pixels_per_point);

    for (id, delta) in output.textures_delta.set {
        ui.renderer.update_ui_texture(id, &delta);
    }

    let screen_descriptor = egui_wgpu::renderer::ScreenDescriptor {
        size_in_pixels: [ui.renderer.config.width, ui.renderer.config.height],
        pixels_per_point: ui.egui_configuration.scale_factor,
    };

    ui.renderer
        .update_ui_buffers(encoder, paint_jobs.as_slice(), &screen_descriptor);

    let mut egui_render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Egui Render Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &texture.surface_tex_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: None,
        occlusion_query_set: None,
        timestamp_writes: None,
    });

    ui.renderer
        .egui_renderer
        .render(&mut egui_render_pass, &paint_jobs, &screen_descriptor);
    ok()
}
