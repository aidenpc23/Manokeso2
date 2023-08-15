use crate::{util::point::Point, camera::Camera};

pub struct BoardView {
    pub pos: Point<f32>,
    pub width: usize,
    pub height: usize,
    pub connex_numbers: Vec<u32>,
    pub stability: Vec<f32>,
    pub reactivity: Vec<f32>,
    pub energy: Vec<f32>,
    pub alpha: Vec<f32>,
    pub beta: Vec<f32>,
    pub gamma: Vec<f32>,
    pub delta: Vec<f32>,
    pub omega: Vec<f32>,
    pub dirty: bool,
    pub total_energy: f32,
}

impl BoardView {
    pub fn empty() -> Self {
        Self {
            pos: Point { x: 0.0, y: 0.0 },
            width: 0,
            height: 0,
            connex_numbers: Vec::new(),
            stability: Vec::new(),
            reactivity: Vec::new(),
            energy: Vec::new(),
            alpha: Vec::new(),
            beta: Vec::new(),
            gamma: Vec::new(),
            delta: Vec::new(),
            omega: Vec::new(),
            dirty: false,
            total_energy: 0.0,
        }
    }
}

pub struct ClientView {
    pub pos: Point<f32>,
    pub width: f32,
    pub height: f32,
}

impl ClientView {
    pub fn new(camera: &Camera) -> Self {
    }
}
