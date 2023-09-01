use std::num::NonZeroU64;

use crate::{client::Camera, common::message::CameraView, util::point::Point};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt, StagingBelt},
    BufferUsages, CommandEncoder, Device,
};
use winit::dpi::PhysicalSize;

const DEFAULT_SCALE: f32 = 0.05;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct CameraUniform {
    pub pos: Point<f32>,
    pub proj: Point<f32>,
}

impl CameraUniform {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn update(&mut self, camera: &Camera, size: &PhysicalSize<u32>) -> bool {
        let new_pos = camera.pos;
        let new_proj = Self::calc_proj(camera, size);
        if self.proj == new_proj && self.pos == new_pos {
            return false;
        }
        self.proj = new_proj;
        self.pos = new_pos;
        true
    }

    fn calc_proj(camera: &Camera, size: &PhysicalSize<u32>) -> Point<f32> {
        let win_aspect = size.width as f32 / size.height as f32;
        let mut proj = if win_aspect > camera.aspect {
            Point::new(1.0, win_aspect)
        } else {
            Point::new(camera.aspect / win_aspect, camera.aspect)
        };
        proj *= camera.scale * DEFAULT_SCALE;
        proj
    }
}

pub struct RenderCamera {
    uniform: CameraUniform,
    buffer: wgpu::Buffer,
}

impl RenderCamera {
    pub fn new(device: &Device) -> Self {
        let uniform = CameraUniform::new();
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        Self { uniform, buffer }
    }
    pub fn update(
        &mut self,
        camera: &Camera,
        window_size: &PhysicalSize<u32>,
        device: &Device,
        encoder: &mut CommandEncoder,
        belt: &mut StagingBelt,
    ) -> Option<CameraView> {
        let mut camera_view = None;
        if self.uniform.update(camera, window_size) {
            let (width, height) = self.world_dimensions();

            camera_view = Some(CameraView {
                pos: self.uniform.pos,
                width,
                height,
            });
            let slice = &[self.uniform];
            let mut view = belt.write_buffer(
                encoder,
                &self.buffer,
                0,
                unsafe {
                    NonZeroU64::new_unchecked(
                        (slice.len() * std::mem::size_of::<CameraUniform>()) as u64,
                    )
                },
                device,
            );
            view.copy_from_slice(bytemuck::cast_slice(slice));
        }
        camera_view
    }
    pub fn binding(&self, binding: u32) -> wgpu::BindGroupEntry<'_> {
        wgpu::BindGroupEntry {
            binding,
            resource: self.buffer.as_entire_binding(),
        }
    }
    pub fn layout_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }
    }

    pub fn world_dimensions(&self) -> (f32, f32) {
        (2.0 / self.uniform.proj.x, 2.0 / self.uniform.proj.y)
    }

    pub fn render_to_world(&self, coords: Point<f32>) -> Point<f32> {
        coords / self.uniform.proj + self.uniform.pos
    }

    pub fn world_to_render(&self, coords: Point<f32>) -> Point<f32> {
        (coords - self.uniform.pos) * self.uniform.proj
    }
}
