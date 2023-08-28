#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextureVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

impl TextureVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<TextureVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub const TEXTURE_VERTICES: &[TextureVertex] = &[
    TextureVertex { position: [0.0, 0.0], tex_coords: [0.0, 1.0], },
    TextureVertex { position: [0.5, 0.0], tex_coords: [1.0, 1.0], },
    TextureVertex { position: [0.0, 0.5], tex_coords: [0.0, 0.0], },
    TextureVertex { position: [0.5, 0.5], tex_coords: [1.0, 0.0], },
];

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ShapeVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

