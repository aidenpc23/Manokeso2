use crate::{camera::Camera, timer::Timer, world::Board};

pub struct GameState {
    pub camera: Camera,
    pub camera_scroll: f32,
    pub board: Board,
    pub timers: Timers,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            camera: Camera::default(),
            camera_scroll: 0.0,
            board: Board::new([-500., -500.], 1000, 1000),
            timers: Timers::new(),
        }
    }
}

pub struct Timers {
    pub update: Timer,
    pub render_extract: Timer,
    pub render_write: Timer,
    pub render_draw: Timer,
    pub total: Timer,
}

impl Timers {
    pub fn new() -> Self {
        let size = 60 * 5;
        Self {
            update: Timer::new(size),
            render_extract: Timer::new(size),
            render_write: Timer::new(size),
            render_draw: Timer::new(size),
            total: Timer::new(size),
        }
    }
}
