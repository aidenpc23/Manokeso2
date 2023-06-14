use std::mem;

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

const SIZE: f32 = 1.0;
const DIST: f32 = SIZE / 2.0;

pub const SQUARE_VERTICES: &[Vertex] = &[
    Vertex {
        position: [-DIST, -DIST],
    },
    Vertex {
        position: [DIST, -DIST],
    },
    Vertex {
        position: [DIST, DIST],
    },
    Vertex {
        position: [-DIST, DIST],
    },
];

pub const SQUARE_INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];
