use std::marker::PhantomData;

use rayon::prelude::*;
use wgpu::{BufferDescriptor, BufferUsages, Device, RenderPass};

use crate::render::writer::StagingBufWriter;

pub struct InstanceField<const LOCATION: u32, T> {
    buffer: wgpu::Buffer,
    label: String,
    len: usize,
    typ: PhantomData<T>,
}

impl<const LOCATION: u32, T: bytemuck::Pod + Send + Default> InstanceField<LOCATION, T> {
    pub fn init(device: &Device, name: &str) -> Self {
        Self {
            label: name.to_owned(),
            len: 0,
            buffer: Self::init_buf(device, name, 0),
            typ: PhantomData {},
        }
    }

    pub fn update_rows(
        &mut self,
        writer: &mut StagingBufWriter,
        row_chunks: rayon::slice::ChunksExact<T>,
        width: usize,
        size: usize,
    ) where
        T: Sync,
    {
        if size != self.len {
            self.len = size;
            self.buffer = Self::init_buf(writer.device, &self.label, self.len);
        }
        if size == 0 {
            return;
        }
        writer
            .mut_view::<T>(&self.buffer, size)
            .par_chunks_exact_mut(width * std::mem::size_of::<T>())
            .zip(row_chunks)
            .for_each(|(data, row)| {
                data.copy_from_slice(bytemuck::cast_slice(&row));
            });
    }

    pub fn set_in<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_vertex_buffer(LOCATION, self.buffer.slice(..));
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn init_buf(device: &Device, label: &str, size: usize) -> wgpu::Buffer {
        device.create_buffer(&BufferDescriptor {
            label: Some(&(label.to_owned() + "Instance Buffer")),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            size: (size * std::mem::size_of::<T>()) as u64,
            mapped_at_creation: false,
        })
    }
}

impl<const LOCATION: u32> InstanceField<LOCATION, u32> {
    pub fn desc(&self) -> wgpu::VertexBufferLayout {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<u32>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &wgpu::vertex_attr_array![LOCATION => Uint32],
        }
    }
}

impl<const LOCATION: u32> InstanceField<LOCATION, f32> {
    pub fn desc(&self) -> wgpu::VertexBufferLayout {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<f32>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &wgpu::vertex_attr_array![LOCATION => Float32],
        }
    }
}
