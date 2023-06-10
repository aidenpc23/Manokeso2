use crate::camera::Camera;

type Vec2<T> = Vec<Vec<T>>;

pub struct GameState {
    pub camera: Camera,
    pub camera_scroll: f32,
    pub colors: Vec2<[f32; 3]>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            camera: Camera::default(),
            camera_scroll: 0.0,
            colors: Vec::new(),
        }
    }
}
