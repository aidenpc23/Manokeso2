use std::num::NonZeroU64;

use wgpu::{util::StagingBelt, BindingResource, Buffer, BufferDescriptor, BufferUsages, Device};

use crate::util::point::Point;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BoardViewUniform {
    pub pos: Point<f32>,
    pub width: u32,
    // shader has an alignment of 8, so we need to add padding
    _padding: u32,
    bruh: [u64; 30],
}

impl BoardViewUniform {
    pub fn new(pos: Point<f32>, width: u32) -> Self {
        Self {
            pos,
            width,
            _padding: 0,
            bruh: [0; 30],
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
        let views = vec![BoardViewUniform::empty(); 1024];
        let len = views.len();
        Self {
            views,
            buffer: Self::init_buf(device, len),
            changed: false,
        }
    }
    pub fn binding(&self) -> BindingResource<'_> {
        BindingResource::Buffer(wgpu::BufferBinding {
            buffer: &self.buffer,
            offset: 0,
            size: NonZeroU64::new(256),
        })
    }
    pub fn init_buf(device: &Device, size: usize) -> Buffer {
        device.create_buffer(&BufferDescriptor {
            label: Some("Board View Buffer"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            size: (size * 256) as u64,
            mapped_at_creation: false,
        })
    }
    pub fn update(&mut self, index: usize, pos: Point<f32>, width: usize) {
        if index + 1 > self.views.len() {
            self.views.resize(index + 1, BoardViewUniform::empty());
        }
        self.changed |= self.views[index].update(pos, width as u32);
    }
    pub fn write(
        &mut self,
        belt: &mut StagingBelt,
        encoder: &mut wgpu::CommandEncoder,
        device: &Device,
    ) {
        let size = (std::mem::size_of::<BoardViewUniform>() * self.views.len()) as u64;
        if size > self.buffer.size() {
            self.buffer = Self::init_buf(device, self.views.len());
        }
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
