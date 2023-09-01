use std::{marker::PhantomData, num::NonZeroU64};

use wgpu::{
    util::StagingBelt, BufferDescriptor, BufferUsages, CommandEncoder, Device, RenderPass,
    VertexAttribute, VertexFormat,
};

pub struct InstanceField<T> {
    pub buffer: wgpu::Buffer,
    label: String,
    pub len: usize,
    pub location: u32,
    attrs: [VertexAttribute; 1],
    _type: PhantomData<T>,
}

impl<T: bytemuck::Pod + Send + Default + Sync> InstanceField<T> {
    pub fn init(device: &Device, name: &str, location: u32, format: VertexFormat) -> Self {
        Self {
            label: name.to_owned(),
            len: 0,
            buffer: Self::init_buf(device, name, 0),
            location,
            attrs: [VertexAttribute {
                format,
                offset: 0,
                shader_location: location,
            }],
            _type: PhantomData {}
        }
    }

    pub fn update_rows(
        &mut self,
        device: &Device,
        encoder: &mut CommandEncoder,
        belt: &mut StagingBelt,
        row_chunks: &[T],
        size: usize,
    ) {
        if size != self.len {
            self.len = size;
            self.buffer = Self::init_buf(device, &self.label, self.len);
        }
        if size == 0 {
            return;
        }
        let mut view = belt.write_buffer(
            encoder,
            &self.buffer,
            0,
            unsafe { NonZeroU64::new_unchecked((size * std::mem::size_of::<T>()) as u64) },
            device,
        );
        view.copy_from_slice(bytemuck::cast_slice(row_chunks));
    }

    pub fn set_in<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_vertex_buffer(self.location, self.buffer.slice(..));
    }

    fn init_buf(device: &Device, label: &str, size: usize) -> wgpu::Buffer {
        device.create_buffer(&BufferDescriptor {
            label: Some(&(label.to_owned() + " Instance Buffer")),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            size: (size * std::mem::size_of::<T>()) as u64,
            mapped_at_creation: false,
        })
    }

    pub fn desc(&self) -> wgpu::VertexBufferLayout {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<T>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &self.attrs,
        }
    }
}
