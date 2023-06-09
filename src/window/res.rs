pub mod square {
    use crate::window::buffer::Vertex;

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
