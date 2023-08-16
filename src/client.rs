use std::{
    sync::{Arc, RwLock, mpsc::Sender},
    time::Duration,
};

use crate::{
    board_view::{BoardView, ClientView},
    camera::Camera,
    config::Config,
    keybinds::{default_keybinds, Keybinds},
    rsc::{FPS, FRAME_TIME, UPDATE_TIME},
    util::{point::Point, timer::Timer}, message::ClientMessage,
};

pub struct Client {
    pub keybinds: Keybinds,
    pub frame_time: Duration,
    pub update_time: Duration,
    pub camera: Camera,
    pub camera_scroll: f32,
    pub held_tile: Option<Point<usize>>,
    pub hovered_tile: Option<Point<usize>>,
    pub paused: bool,
    pub step: bool,
    pub frame_timer: Timer,
    pub board_view: Arc<RwLock<BoardView>>,
    pub client_view: Arc<RwLock<ClientView>>,
    pub sender: Sender<ClientMessage>
}

impl Client {
    pub fn new(config: Config, sender: Sender<ClientMessage>) -> Self {
        let mut keybinds = default_keybinds();
        if let Some(config_keybinds) = config.keybinds {
            keybinds.extend(config_keybinds);
        }
        let camera = Camera::default();
        Self {
            keybinds,
            frame_time: FRAME_TIME,
            update_time: UPDATE_TIME,
            camera,
            camera_scroll: 0.0,
            held_tile: None,
            hovered_tile: None,
            paused: true,
            step: false,
            frame_timer: Timer::new(FPS as usize),
            board_view: Arc::new(BoardView::empty().into()),
            client_view: Arc::new(ClientView::new().into()),
            sender
        }
    }
}
