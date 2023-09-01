use std::num::NonZeroU64;

use wgpu::{util::StagingBelt, BindingResource, Buffer, BufferDescriptor, BufferUsages, Device};

use crate::util::point::Point;

const ALIGN: u64 = 256;
const MAX: usize = 1000;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BoardViewUniform {
    pub pos: Point<f32>,
    pub width: u32,
    /// shader has an alignment of 8, so we need to add padding
    _padding: u32,
    /// dynamic uniform buffers have min alignment of 256 (as of august 23rd 2023)
    _padding2: [u64; 30],
}

impl BoardViewUniform {
    pub fn new(pos: Point<f32>, width: u32) -> Self {
        Self {
            pos,
            width,
            _padding: 0,
            _padding2: [0; 30],
        }
    }
    pub fn update(&mut self, pos: Point<f32>, width: u32) -> bool {
        if self.pos == pos && self.width == width {
            return false;
        }
        self.pos = pos;
        self.width = width;
        true
    }
    pub fn empty() -> Self {
        Self::new(Point::zero(), 0)
    }
}

pub struct BoardViews {
    views: Vec<BoardViewUniform>,
    buffer: wgpu::Buffer,
    changed: bool,
}

impl BoardViews {
    pub fn new(device: &Device) -> Self {
        let views = vec![BoardViewUniform::empty(); MAX];
        let len = views.len();
        Self {
            views,
            buffer: Self::init_buf(device, len),
            changed: false,
        }
    }
    pub fn binding(&self, binding: u32) -> wgpu::BindGroupEntry<'_> {
        wgpu::BindGroupEntry {
            binding,
            resource: BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &self.buffer,
                offset: 0,
                size: NonZeroU64::new(ALIGN),
            }),
        }
    }
    pub fn layout_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: true,
                min_binding_size: None,
            },
            count: None,
        }
    }
    pub fn init_buf(device: &Device, size: usize) -> Buffer {
        device.create_buffer(&BufferDescriptor {
            label: Some("Board View Buffer"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            size: size as u64 * ALIGN,
            mapped_at_creation: false,
        })
    }
    pub fn update(&mut self, index: usize, pos: Point<f32>, width: usize) {
        if index >= MAX {
            println!("tried to write too many boards!");
            return;
        }
        self.changed |= self.views[index].update(pos, width as u32);
    }
    pub fn write(
        &mut self,
        belt: &mut StagingBelt,
        encoder: &mut wgpu::CommandEncoder,
        device: &Device,
    ) {
        let size = (std::mem::size_of::<BoardViewUniform>() * self.views.len().min(MAX)) as u64;
        if size != 0 {
            let mut view = belt.write_buffer(
                encoder,
                &self.buffer,
                0,
                unsafe { NonZeroU64::new_unchecked(size) },
                device,
            );
            view.copy_from_slice(bytemuck::cast_slice(&self.views[..]));
        }
    }
}
