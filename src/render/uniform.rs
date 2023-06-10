use winit::dpi::PhysicalSize;

use crate::camera::Camera;

const DEFAULT_SCALE: f32 = 0.05;

#[repr(C)]
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

    pub fn bottom_left(&self) -> [f32; 2] {
        self.cam_to_world([-1.0, -1.0])
    }

    pub fn top_right(&self) -> [f32; 2] {
        self.cam_to_world([1.0, 1.0])
    }

    fn cam_to_world(&self, mut coords: [f32; 2]) -> [f32; 2] {
        coords[0] /= self.proj[0];
        coords[1] /= self.proj[1];
        coords[0] += self.pos[0];
        coords[1] += self.pos[1];
        coords
    }
}

impl PartialEq for CameraUniform {
    fn eq(&self, other: &Self) -> bool {
        arr_eq(self.proj, other.proj) && arr_eq(self.pos, other.pos)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TileViewUniform {
    pub pos: [f32; 2],
    pub width: u32,
    // shader has an alignment of 8, so we need to add padding
    _padding: u32,
}

impl TileViewUniform {
    pub fn new(pos: [f32; 2], width: u32) -> Self {
        Self {
            pos,
            width,
            _padding: 0,
        }
    }
}

impl PartialEq for TileViewUniform {
    fn eq(&self, other: &Self) -> bool {
        arr_eq(self.pos, other.pos) && self.width == other.width
    }
}

fn arr_eq<T: PartialEq, const N: usize>(arr1: [T; N], arr2: [T; N]) -> bool {
    arr1.iter().zip(arr2.iter()).all(|(x, y)| x == y)
}
