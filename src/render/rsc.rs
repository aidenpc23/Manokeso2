pub const CLEAR_COLOR: wgpu::Color = wgpu::Color {
    r: 0.1,
    g: 0.1,
    b: 0.1,
    a: 1.0,
};

pub mod square {
    use crate::render::buffer::Vertex;

    pub const VERTICES: &[Vertex] = &[
        Vertex {
            position: [-0.5, -0.5],
        },
        Vertex {
            position: [0.5, -0.5],
        },
        Vertex {
            position: [0.5, 0.5],
        },
        Vertex {
            position: [-0.5, 0.5],
        },
    ];

    pub const INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];
}
