use wgpu::{RenderPass, VertexAttribute};

use crate::render::{primitive::RoundedRectInstance, surface::RenderSurface};

pub struct RoundedRectBuffer {
    buffer: wgpu::Buffer,
    len: usize,
}

impl RoundedRectBuffer {
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            buffer: Self::init_buf(device, 0),
            len: 0,
        }
    }
    pub fn update(&mut self, surface: &RenderSurface, rects: &[RoundedRectInstance]) {
        if self.len != rects.len() {
            self.len = rects.len();
            self.buffer = Self::init_buf(
                &surface.device,
                rects.len() * std::mem::size_of::<RoundedRectInstance>(),
            );
        }
        surface
            .queue
            .write_buffer(&self.buffer, 0, bytemuck::cast_slice(rects));
    }
    fn init_buf(device: &wgpu::Device, size: usize) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance Buffer"),
            size: size as u64,
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        })
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn set_in<'a>(&'a self, pass: &mut RenderPass<'a>) {
        pass.set_vertex_buffer(0, self.buffer.slice(..));
    }
}

impl RoundedRectInstance {
    const ATTRIBS: [VertexAttribute; 11] = wgpu::vertex_attr_array![
        0 => Float32x2,
        1 => Float32x2,
        2 => Float32x2,
        3 => Float32x2,
        4 => Float32x4,
        5 => Float32x4,
        6 => Float32x4,
        7 => Float32x4,
        8 => Float32,
        9 => Float32,
        10 => Float32,
    ];
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<RoundedRectInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}

