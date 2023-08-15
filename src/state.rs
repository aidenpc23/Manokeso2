use std::time::Duration;

use crate::{
    camera::Camera,
    config::Config,
    timer::Timer,
    rsc::{FRAME_TIME, UPDATE_TIME}, keybinds::{Keybinds, default_keybinds}, util::point::Point, tile_view::BoardView,
};

pub struct GameState {
    pub keybinds: Keybinds,
    pub frame_time: Duration,
    pub update_time: Duration,
    pub camera: Camera,
    pub camera_scroll: f32,
    pub held_tile: Option<Point<usize>>,
    pub hovered_tile: Option<Point<usize>>,
    pub paused: bool,
    pub step: bool,
    pub timers: Timers,
    pub tile_view: BoardView,
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
            held_tile: None,
            hovered_tile: None,
            paused: true,
            step: false,
            timers: Timers::new(),
            tile_view: BoardView::empty(),
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
