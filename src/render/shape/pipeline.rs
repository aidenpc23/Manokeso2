use wgpu::{BindGroup, RenderPipeline, Buffer, RenderPass};

use crate::render::surface::RenderSurface;

use super::uniform::WindowUniform;

pub const SHAPE_SHADER: &str = include_str!("./shader.wgsl");

pub struct ShapeBuffers {
    pub window: Buffer,
    pub instance: Buffer
}

pub struct ShapePipeline {
    pub bind_group: BindGroup,
    pub pipeline: RenderPipeline,

    pub buffers: ShapeBuffers,
}

impl ShapePipeline {
    pub fn draw<'a>(&'a self, pass: &mut RenderPass<'a>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.set_vertex_buffer(0, self.buffers.instance.slice(..));
        pass.draw(0..4, 0..1);
    }

    pub fn update(
        &mut self,
        surface: &RenderSurface,
    ) {
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
