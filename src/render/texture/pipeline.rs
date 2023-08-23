use wgpu::{RenderPass, RenderPipeline};

pub const TEXTURE_SHADER: &str = include_str!("./shader.wgsl");

pub struct TexturePipeline {
    pub pipeline: RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub diffuse_bind_group: wgpu::BindGroup,
}

impl TexturePipeline {
    pub fn draw<'a>(&'a self, pass: &mut RenderPass<'a>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.draw(0..4, 0..1);
    }
}
