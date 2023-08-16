use std::{
    sync::{mpsc::Sender, Arc},
    time::Duration,
};

use crate::{
    board_view::{BoardView, BoardViewInfo, BoardViewLock},
    camera::Camera,
    config::Config,
    keybinds::{default_keybinds, Keybinds},
    message::ClientMessage,
    rsc::{FPS, FRAME_TIME},
    util::{point::Point, timer::Timer},
};

pub struct ClientState {
    pub keybinds: Keybinds,
    pub frame_time: Duration,
    pub camera: Camera,
    pub camera_scroll: f32,
    pub held_tile: Option<TileInfo>,
    pub hovered_tile: Option<TileInfo>,
    pub paused: bool,
    pub frame_timer: Timer,
    pub board_view: BoardViewLock,
    pub view_info: BoardViewInfo,
    pub sender: Sender<ClientMessage>,
}

impl ClientState {
    pub fn new(config: Config, sender: Sender<ClientMessage>) -> Self {
        let mut keybinds = default_keybinds();
        if let Some(config_keybinds) = config.keybinds {
            keybinds.extend(config_keybinds);
        }
        let camera = Camera::default();
        let view = BoardView::empty();
        let info = view.info.clone();
        Self {
            keybinds,
            frame_time: FRAME_TIME,
            camera,
            camera_scroll: 0.0,
            held_tile: None,
            hovered_tile: None,
            paused: true,
            frame_timer: Timer::new(FPS as usize),
            board_view: Arc::new(view.into()),
            view_info: info,
            sender,
        }
    }

    pub fn send(&self, message: ClientMessage) {
        if let Err(err) = self.sender.send(message) {
            println!("Failed to send message to server: {:?}", err);
        }
    }
}

#[derive(Clone, Copy)]
pub struct TileInfo {
    pub pos: Point<usize>,
    pub connex_number: u32,
    pub stability: f32,
    pub reactivity: f32,
    pub energy: f32,
    pub alpha: u64,
    pub beta: u64,
    pub gamma: f32,
    pub delta: f32,
    pub omega: f32,
}
