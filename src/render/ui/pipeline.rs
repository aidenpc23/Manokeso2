use wgpu::{RenderPass, RenderPipeline, Buffer, BindGroup};
use winit::dpi::PhysicalSize;

use crate::render::surface::RenderSurface;

use super::{text::{TextElement, UIText}, uniform::WindowUniform};

pub const TEXTURE_SHADER: &str = include_str!("./shader.wgsl");
pub const SHAPE_SHADER: &str = include_str!("./shader2.wgsl");

pub struct UIPipeline {
    pub texture_pipeline: RenderPipeline,
    pub texture_vertex_buffer: wgpu::Buffer,
    pub diffuse_bind_group: wgpu::BindGroup,

    pub shape_bind_group: BindGroup,
    pub shape_pipeline: RenderPipeline,

    pub buffers: UIBuffers,

    pub text: UIText,
}

pub struct UIBuffers {
    pub window: Buffer
}

impl UIPipeline {
    pub fn draw<'a>(&'a self, pass: &mut RenderPass<'a>) {
        pass.set_pipeline(&self.shape_pipeline);
        pass.set_bind_group(0, &self.shape_bind_group, &[]);
        pass.draw(0..4, 0..1);

        // pass.set_pipeline(&self.texture_pipeline);
        // pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
        // pass.set_vertex_buffer(0, self.texture_vertex_buffer.slice(..));
        // pass.draw(0..4, 0..1);

        self.text.draw(pass);
    }

    pub fn update(
        &mut self,
        size: &PhysicalSize<u32>,
        surface: &RenderSurface,
        text: &[TextElement],
    ) {
        self.text.update(size, surface, text);

        let slice = &[WindowUniform {
            width: surface.config.width,
            height: surface.config.height
        }];
        surface.queue.write_buffer(
            &self.buffers.window,
            0,
            bytemuck::cast_slice(slice)
        );
    }
}
