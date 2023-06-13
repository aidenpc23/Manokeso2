use std::mem;

use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BufferUsages, Device, Queue,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![
        0 => Float32x2
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub struct Instance<T> {
    pub label: String,
    pub data_len: usize,
    pub data: Vec<T>,
    pub buffer: wgpu::Buffer,
}

impl<T: bytemuck::Pod> Instance<T> {
    pub fn init(device: &Device, name: &str) -> Self {
        Self {
            label: name.to_owned(),
            data_len: 0,
            data: Vec::new(),
            buffer: Self::init_buf(device, name, &[]),
        }
    }

    pub fn write(&mut self, device: &Device, queue: &Queue, len: usize) {
        if self.data_len != len {
            self.data_len = len;
            self.buffer = Self::init_buf(device, &self.label, &self.data[0..self.data_len]);
        } else {
            queue.write_buffer(
                &self.buffer,
                0,
                bytemuck::cast_slice(&self.data[0..self.data_len]),
            );
        }
    }

    fn init_buf(device: &Device, label: &str, contents: &[T]) -> wgpu::Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: Some(&(label.to_owned() + "Instance Buffer")),
            contents: bytemuck::cast_slice(contents),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        })
    }
}

pub fn instance_descs() -> Vec<wgpu::VertexBufferLayout<'static>> {
    vec![
        wgpu::VertexBufferLayout {
            array_stride: 4 as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &wgpu::vertex_attr_array![1 => Uint32],
        },
        wgpu::VertexBufferLayout {
            array_stride: 4 as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &wgpu::vertex_attr_array![2 => Float32],
        },
        wgpu::VertexBufferLayout {
            array_stride: 4 as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &wgpu::vertex_attr_array![3 => Float32],
        },
        wgpu::VertexBufferLayout {
            array_stride: 4 as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &wgpu::vertex_attr_array![4 => Float32],
        },
    ]
}
