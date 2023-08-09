use std::time::Duration;

use crate::{
    camera::Camera,
    config::Config,
    timer::Timer,
    world::Board, rsc::{FRAME_TIME, UPDATE_TIME}, keybinds::{Keybinds, default_keybinds},
};

pub struct GameState {
    pub keybinds: Keybinds,
    pub frame_time: Duration,
    pub update_time: Duration,
    pub camera: Camera,
    pub camera_scroll: f32,
    pub board: Board,
    pub selected_tile: Option<[usize; 2]>,
    pub paused: bool,
    pub step: bool,
    pub timers: Timers,
}

impl GameState {
    pub fn new(config: Config) -> Self {
        let mut keybinds = default_keybinds();
        if let Some(config_keybinds) = config.keybinds {
            keybinds.extend(config_keybinds);
        }
        Self {
            keybinds,
            frame_time: FRAME_TIME,
            update_time: UPDATE_TIME,
            camera: Camera::default(),
            camera_scroll: 0.0,
            board: Board::new([-500., -500.], 1000, 1000),
            selected_tile: None,
            paused: false,
            step: false,
            timers: Timers::new(),
        }
    }
}

pub struct Timers {
    pub update: Timer,
    pub render_extract: Timer,
    pub render_write: Timer,
    pub render_draw: Timer,
}

impl Timers {
    pub fn new() -> Self {
        let size = 60 * 5;
        Self {
            update: Timer::new(size),
            render_extract: Timer::new(size),
            render_write: Timer::new(size),
            render_draw: Timer::new(size),
        }
    }
}
