#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FrameUniform {
    pub time: u64,
}

impl FrameUniform {
    pub fn new() -> Self {
        let a = std::time::Instant::now();
        Self { time: 0 }
    }
}
