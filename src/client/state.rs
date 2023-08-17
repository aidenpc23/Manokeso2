use std::{
    sync::{
        mpsc::{Receiver, Sender},
        Arc,
    },
    time::Duration,
};

use super::{
    camera::Camera,
    config::Config,
    keybinds::{default_keybinds, Keybinds},
};

use crate::{
    message::{ClientMessage, WorldMessage},
    rsc::{FPS, FRAME_TIME},
    util::timer::Timer, sync::{TileInfo, WorldInterface, BoardView},
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
    pub world: WorldInterface,
}

impl ClientState {
    pub fn new(
        config: Config,
        sender: Sender<ClientMessage>,
        receiver: Receiver<WorldMessage>,
    ) -> Self {
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
            frame_timer: Timer::new(Duration::from_secs(1), FPS as usize),
            world: WorldInterface {
                sender,
                receiver,
                view_lock: Arc::new(view.into()),
                view_info: info,
            },
        }
    }
}

