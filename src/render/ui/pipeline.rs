use wgpu::{RenderPass, RenderPipeline, TextureView};
use wgpu_glyph::GlyphBrush;
use winit::dpi::PhysicalSize;

use crate::{render::writer::StagingBufWriter, state::GameState};

use super::layout::{create_sections, UIText};

pub const SHADER: &str = concat!(include_str!("./shader.wgsl"));

pub struct UIPipeline {
    pub(super) pipeline: RenderPipeline,
    pub brush: GlyphBrush<()>,
    pub text: UIText,
    pub vertex_buffer: wgpu::Buffer,
    pub diffuse_bind_group: wgpu::BindGroup,
}

impl UIPipeline {
    pub fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..4, 0..1);
    }

    pub fn update(&mut self, state: &GameState, size: &PhysicalSize<u32>) {
        for section in create_sections(state, &mut self.text, (size.width as f32, size.height as f32)) {
            self.brush.queue(section);
        }
    }

    pub fn draw_text(
        &mut self,
        writer: &mut StagingBufWriter,
        view: &TextureView,
        size: &PhysicalSize<u32>,
    ) {
        self.brush
            .draw_queued(
                writer.device,
                writer.belt,
                writer.encoder,
                view,
                size.width,
                size.height,
            )
            .expect("Draw queued");
    }
}
