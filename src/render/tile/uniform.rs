use winit::dpi::PhysicalSize;

use crate::{
    client::Camera,
    rsc::{CONNEX_NUMBER_RANGE, ENERGY_RANGE, REACTIVITY_RANGE, STABILITY_RANGE},
    util::point::Point,
};

const DEFAULT_SCALE: f32 = 0.05;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct CameraUniform {
    pub pos: Point<f32>,
    pub proj: Point<f32>,
}

impl CameraUniform {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn update(&mut self, camera: &Camera, size: &PhysicalSize<u32>) -> bool {
        let new_pos = camera.pos;
        let new_proj = Self::calc_proj(camera, size);
        if self.proj == new_proj && self.pos == new_pos {
            return false;
        }
        self.proj = new_proj;
        self.pos = new_pos;
        true
    }

    pub fn world_dimensions(&self) -> (f32, f32) {
        (2.0 / self.proj.x, 2.0 / self.proj.y)
    }

    pub fn render_to_world(&self, coords: Point<f32>) -> Point<f32> {
        coords / self.proj + self.pos
    }

    fn calc_proj(camera: &Camera, size: &PhysicalSize<u32>) -> Point<f32> {
        let win_aspect = size.width as f32 / size.height as f32;
        let mut proj = if win_aspect > camera.aspect {
            Point::new(1.0, win_aspect)
        } else {
            Point::new(camera.aspect / win_aspect, camera.aspect)
        };
        proj *= camera.scale * DEFAULT_SCALE;
        proj
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TileViewUniform {
    pub pos: Point<f32>,
    pub width: u32,
    // shader has an alignment of 8, so we need to add padding
    _padding: u32,
}

impl TileViewUniform {
    pub fn new(pos: Point<f32>, width: u32) -> Self {
        Self {
            pos,
            width,
            _padding: 0,
        }
    }
    pub fn update(&mut self, pos: Point<f32>, width: u32) -> bool {
        if self.pos == pos && self.width == width {
            return false;
        }
        self.pos = pos;
        self.width = width;
        true
    }
    pub fn empty() -> Self {
        Self::new(Point::zero(), 0)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ConstsUniform {
    pub connex_number_range: [u32; 2],
    pub stability_range: [f32; 2],
    pub reactivity_range: [f32; 2],
    pub energy_range: [f32; 2],
}

impl ConstsUniform {
    pub fn new() -> Self {
        Self {
            connex_number_range: CONNEX_NUMBER_RANGE,
            stability_range: STABILITY_RANGE,
            reactivity_range: REACTIVITY_RANGE,
            energy_range: ENERGY_RANGE,
        }
    }
}
