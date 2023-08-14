use std::time::Duration;

use crate::{
    camera::Camera,
    config::Config,
    timer::Timer,
    world::Board, rsc::{FRAME_TIME, UPDATE_TIME}, keybinds::{Keybinds, default_keybinds}, util::point::Point,
};

pub struct GameState {
    pub keybinds: Keybinds,
    pub frame_time: Duration,
    pub update_time: Duration,
    pub camera: Camera,
    pub camera_scroll: f32,
    pub board: Board,
    pub held_tile: Option<[usize; 2]>,
    pub hovered_tile: Option<[usize; 2]>,
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
        let width = 1000;
        let height = 1000;
        Self {
            keybinds,
            frame_time: FRAME_TIME,
            update_time: UPDATE_TIME,
            camera: Camera::default(),
            camera_scroll: 0.0,
            board: Board::new(Point::new(-(width as f32) / 2.0, -(height as f32) / 2.0), width, height),
            held_tile: None,
            hovered_tile: None,
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
