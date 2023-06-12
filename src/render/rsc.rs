pub const CLEAR_COLOR: wgpu::Color = wgpu::Color {
    r: 0.1,
    g: 0.1,
    b: 0.1,
    a: 1.0,
};

pub mod square {
    const SIZE: f32 = 1.0;
    const DIST: f32 = SIZE / 2.0;
    use crate::render::buffer::Vertex;

    pub const VERTICES: &[Vertex] = &[
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

    pub const INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];
}
