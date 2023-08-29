use wgpu::{BindGroup, Buffer, RenderPass, RenderPipeline};

use crate::render::{primitive::RoundedRectPrimitive, surface::RenderSurface};

use super::{uniform::WindowUniform, instance::RoundedRectBuffer};

pub const SHAPE_SHADER: &str = include_str!("./shader.wgsl");

pub struct ShapeBuffers {
    pub window: Buffer,
    pub instance: RoundedRectBuffer,
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
        self.buffers.instance.set_in(pass);
        pass.draw(0..4, 0..self.buffers.instance.len() as u32);
    }

    pub fn update(
        &mut self,
        surface: &RenderSurface,
        rects: &Vec<RoundedRectPrimitive>,
        resized: bool,
    ) {
        if resized {
            let slice = &[WindowUniform {
                width: surface.config.width as f32,
                height: surface.config.height as f32,
            }];
            surface
                .queue
                .write_buffer(&self.buffers.window, 0, bytemuck::cast_slice(slice));
        }
        self.buffers.instance.update(surface, rects);
    }
}
