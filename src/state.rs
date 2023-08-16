use std::{
    sync::{mpsc::Sender, Arc, RwLock},
    time::{Duration, Instant},
};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::{
    board_view::BoardView,
    camera::Camera,
    config::Config,
    keybinds::{default_keybinds, Keybinds},
    message::ClientMessage,
    rsc::{FPS, FRAME_TIME, UPDATE_TIME},
    util::{point::Point, timer::Timer}, input::Input, handle_input::handle_input, render::Renderer,
};

pub struct ClientState {
    pub keybinds: Keybinds,
    pub frame_time: Duration,
    pub update_time: Duration,
    pub camera: Camera,
    pub camera_scroll: f32,
    pub held_tile: Option<Point<usize>>,
    pub hovered_tile: Option<Point<usize>>,
    pub paused: bool,
    pub frame_timer: Timer,
    pub board_view: Arc<RwLock<BoardView>>,
    pub sender: Sender<ClientMessage>,
}

impl ClientState {
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
            frame_timer: Timer::new(FPS as usize),
            board_view: Arc::new(BoardView::empty().into()),
            sender,
        }
    }

    pub fn run(mut self, mut renderer: Renderer, event_loop: EventLoop<()>) {
    }

    pub fn send(&self, message: ClientMessage) {
        if let Err(err) = self.sender.send(message) {
            println!("Failed to send message to server: {:?}", err);
        }
    }
}
