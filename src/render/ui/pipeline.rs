use wgpu::{RenderPass, RenderPipeline};
use winit::dpi::PhysicalSize;

use crate::render::surface::RenderSurface;

use super::text::{UIText, TextElement};

pub const SHADER: &str = concat!(include_str!("./shader.wgsl"));

pub struct UIPipeline {
    pub(super) pipeline: RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub diffuse_bind_group: wgpu::BindGroup,
    pub text: UIText,
}

impl UIPipeline {
    pub fn draw<'a>(&'a self, pass: &mut RenderPass<'a>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        // pass.draw(0..4, 0..1);

        self.text.draw(pass);
    }

    pub fn update(
        &mut self,
        size: &PhysicalSize<u32>,
        surface: &RenderSurface,
        text: &[TextElement]
    ) {
        self.text.update(size, surface, text);
    }
}
