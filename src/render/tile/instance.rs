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
    pub fn init(device: &Device, name: &str, location: u32) -> Self
    where
        T: InstanceFieldType,
    {
        Self {
            label: name.to_owned(),
            len: 0,
            buffer: Self::init_buf(device, name, 0),
            location,
            attrs: [VertexAttribute {
                format: T::format(),
                offset: 0,
                shader_location: location,
            }],
            _type: PhantomData {},
        }
    }

    pub fn update_rows(
        &mut self,
        device: &Device,
        encoder: &mut CommandEncoder,
        belt: &mut StagingBelt,
        row_chunks: std::slice::ChunksExact<T>,
        width: usize,
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
        view.chunks_exact_mut(width * std::mem::size_of::<T>())
            .zip(row_chunks)
            .for_each(|(data, row)| {
                data.copy_from_slice(bytemuck::cast_slice(&row));
            });
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

pub trait InstanceFieldType {
    type RustType;
    fn format() -> VertexFormat
    where
        Self: Sized;
}

impl InstanceFieldType for f32 {
    type RustType = f32;
    fn format() -> VertexFormat {
        VertexFormat::Float32
    }
}

impl InstanceFieldType for u32 {
    type RustType = u32;
    fn format() -> VertexFormat {
        VertexFormat::Uint32
    }
}
