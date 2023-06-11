use crate::{camera::Camera, world::Board};

pub struct GameState {
    pub camera: Camera,
    pub camera_scroll: f32,
    pub board: Board,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            camera: Camera::default(),
            camera_scroll: 0.0,
            board: Board::new(1000, 1000),
        }
    }
}
