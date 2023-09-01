use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BufferUsages, Device,
};

use crate::rsc::{CONNEX_NUMBER_RANGE, ENERGY_RANGE, REACTIVITY_RANGE, STABILITY_RANGE};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ConstsUniform {
    pub connex_number_range: [u32; 2],
    pub stability_range: [f32; 2],
    pub reactivity_range: [f32; 2],
    pub energy_range: [f32; 2],
}

impl ConstsUniform {
    pub fn new() -> Self {
        Self {
            connex_number_range: CONNEX_NUMBER_RANGE,
            stability_range: STABILITY_RANGE,
            reactivity_range: REACTIVITY_RANGE,
            energy_range: ENERGY_RANGE,
        }
    }
}

pub struct RenderConsts {
    buffer: wgpu::Buffer,
}

impl RenderConsts {
    pub fn new(device: &Device) -> Self {
        let uniform = ConstsUniform::new();
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Constants Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: BufferUsages::UNIFORM,
        });

        Self { buffer }
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
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }
    }
}
