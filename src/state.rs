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
            board: Board::new([-350., -350.], 708, 708),
            selected_tile: None,
            paused: true,
            step: false,
            timers: Timers::new(),
        }
    }
}

pub struct Timers {
    pub update: Timer,
    pub render: Timer,
}

impl Timers {
    pub fn new() -> Self {
        let size = 60;
        Self {
            update: Timer::new(size),
            render: Timer::new(size),
        }
    }
}
