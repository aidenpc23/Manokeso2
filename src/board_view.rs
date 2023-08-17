use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::{util::point::Point, world::BoardSlice};

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
    pub pos: Point<f32>,
    pub slice: BoardSlice,
    pub dirty: bool,
    pub total_energy: f32,
    pub time_taken: Duration,
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
                pos: Point { x: 0.0, y: 0.0 },
                slice: BoardSlice::default(),
                dirty: false,
                total_energy: 0.0,
                time_taken: Duration::ZERO,
            },
        }
    }
}
