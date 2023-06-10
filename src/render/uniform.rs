use winit::dpi::PhysicalSize;

use crate::camera::Camera;

const DEFAULT_SCALE: f32 = 0.05;

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pos: [f32; 2],
    proj: [f32; 2],
}

impl CameraUniform {
    pub fn new(camera: &Camera, size: &PhysicalSize<u32>) -> Self {
        let win_aspect = size.width as f32 / size.height as f32;
        let mut proj = if win_aspect > camera.aspect {
            [1.0, win_aspect]
        } else {
            [camera.aspect / win_aspect, camera.aspect]
        };
        proj[0] *= camera.scale * DEFAULT_SCALE;
        proj[1] *= camera.scale * DEFAULT_SCALE;
        Self {
            pos: camera.pos,
            proj,
        }
    }
}

impl PartialEq for CameraUniform {
    fn eq(&self, other: &Self) -> bool {
        arr_eq(self.proj, other.proj) && arr_eq(self.pos, other.pos)
    }
}

fn arr_eq<T: PartialEq, const N: usize>(arr1: [T; N], arr2: [T; N]) -> bool {
    arr1.iter().zip(arr2.iter()).all(|(x, y)| x == y)
}
