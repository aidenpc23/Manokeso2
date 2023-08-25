use std::{
    sync::{Arc, Mutex, mpsc::{Sender, Receiver}},
    time::Duration,
};

use crate::{util::point::Point, message::{ClientMessage, WorldMessage}, render::tile::data::RenderViewInfo};

pub type BoardViewLock = Arc<Mutex<BoardView>>;

pub struct BoardView {
    pub connex_numbers: Vec<u32>,
    pub stability: Vec<f32>,
    pub reactivity: Vec<f32>,
    pub energy: Vec<f32>,
    pub alpha: Vec<u64>,
    pub beta: Vec<u64>,
    pub gamma: Vec<f32>,
    pub delta: Vec<f32>,
    pub omega: Vec<f32>,
    pub info: BoardViewInfo,
}

#[derive(Clone, Copy)]
pub struct BoardViewInfo {
    pub render_info: RenderViewInfo,
    pub total_energy: f32,
    pub time_taken: Duration,
    pub pos: Point<f32>,
}

impl BoardView {
    pub fn empty() -> Self {
        Self {
            connex_numbers: Vec::new(),
            stability: Vec::new(),
            reactivity: Vec::new(),
            energy: Vec::new(),
            alpha: Vec::new(),
            beta: Vec::new(),
            gamma: Vec::new(),
            delta: Vec::new(),
            omega: Vec::new(),
            info: BoardViewInfo {
                render_info: RenderViewInfo::new(),
                total_energy: 0.0,
                time_taken: Duration::ZERO,
                pos: Point::zero(),
            },
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

pub struct WorldInterface {
    pub sender: Sender<ClientMessage>,
    pub receiver: Receiver<WorldMessage>,
    pub view_lock: BoardViewLock,
    pub view_info: BoardViewInfo,
}

impl WorldInterface {
    pub fn send(&self, message: ClientMessage) {
        if let Err(err) = self.sender.send(message) {
            println!("Failed to send message to server: {:?}", err);
        }
    }
}
