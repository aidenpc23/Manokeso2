#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct WindowUniform {
    pub width: u32,
    pub height: u32,
}

impl WindowUniform {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }
}
