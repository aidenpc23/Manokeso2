use wgpu::VertexAttribute;

use crate::util::point::Point;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ShapeInstance {
    pub top_left: Point<f32>,
    pub bottom_right: Point<f32>,
    pub radius: f32,
    pub inner_radius: f32,
    pub thickness: f32,
}

impl ShapeInstance {
    const ATTRIBS: [VertexAttribute; 5] = wgpu::vertex_attr_array![
        0 => Float32x2,
        1 => Float32x2,
        2 => Float32,
        3 => Float32,
        4 => Float32,
    ];
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ShapeInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}
