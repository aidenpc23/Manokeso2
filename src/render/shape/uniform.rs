#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct WindowUniform {
    pub width: f32,
    pub height: f32,
}

impl WindowUniform {
    pub fn new() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
        }
    }
}
