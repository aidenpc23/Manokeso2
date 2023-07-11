use rayon::prelude::*;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BufferUsages, Device, Queue, RenderPass,
};

pub struct InstanceField<const LOCATION: u32, T> {
    label: String,
    len: usize,
    data: Vec<T>,
    buffer: wgpu::Buffer,
    recreate_buf: bool,
}

impl<const LOCATION: u32, T: bytemuck::Pod + Send + Default> InstanceField<LOCATION, T> {
    pub fn init(device: &Device, name: &str) -> Self {
        Self {
            label: name.to_owned(),
            len: 0,
            data: Vec::new(),
            buffer: Self::init_buf(device, name, &[]),
            recreate_buf: false,
        }
    }

    pub fn write_buf(&mut self, device: &Device, queue: &Queue) {
        if self.recreate_buf {
            self.buffer = Self::init_buf(device, &self.label, &self.data[0..self.len]);
        } else {
            queue.write_buffer(
                &self.buffer,
                0,
                bytemuck::cast_slice(&self.data[0..self.len]),
            );
        }
    }

    pub fn update_rows(
        &mut self,
        row_chunks: rayon::slice::ChunksExact<T>,
        size: usize,
        xs: usize,
        xe: usize,
    ) where
        T: Sync,
    {
        let width = xe - xs;
        if size != self.data.len() {
            self.data.resize(size, Default::default());
            self.len = size;
            self.recreate_buf = true;
        }
        if size == 0 {
            return;
        }
        self.data[0..size]
            .par_chunks_exact_mut(width)
            .zip(row_chunks)
            .for_each(|(data, row)| {
                data.copy_from_slice(&row[xs..xe]);
            });
    }

    pub fn set_in<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_vertex_buffer(LOCATION, self.buffer.slice(..));
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn init_buf(device: &Device, label: &str, contents: &[T]) -> wgpu::Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: Some(&(label.to_owned() + "Instance Buffer")),
            contents: bytemuck::cast_slice(contents),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
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
