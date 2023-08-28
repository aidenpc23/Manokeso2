use std::{
    sync::mpsc::{Receiver, Sender},
    time::Duration,
};

use crate::{
    message::{WorkerCommand, WorkerResponse},
    util::point::Point,
};

#[derive(Debug)]
pub struct BoardView {
    pub board_pos: Point<f32>,

    pub connex_numbers: Vec<u32>,
    pub stability: Vec<f32>,
    pub reactivity: Vec<f32>,
    pub energy: Vec<f32>,
    pub alpha: Vec<u64>,
    pub beta: Vec<u64>,
    pub gamma: Vec<f32>,
    pub delta: Vec<u64>,
    pub omega: Vec<f32>,

    pub slice: BoardSlice,
    pub total_energy: f32,
    pub time_taken: Duration,
}

impl BoardView {
    pub fn empty() -> Self {
        Self {
            board_pos: Point::zero(),
            connex_numbers: Vec::new(),
            stability: Vec::new(),
            reactivity: Vec::new(),
            energy: Vec::new(),
            alpha: Vec::new(),
            beta: Vec::new(),
            gamma: Vec::new(),
            delta: Vec::new(),
            omega: Vec::new(),
            slice: BoardSlice::empty(),
            total_energy: 0.0,
            time_taken: Duration::ZERO,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct BoardSlice {
    pub world_pos: Point<f32>,
    pub start: Point<usize>,
    pub end: Point<usize>,
    pub width: usize,
    pub height: usize,
    pub size: usize,
}

impl BoardSlice {
    pub fn new(world_pos: Point<f32>, start: Point<usize>, end: Point<usize>) -> Self {
        let diff = end - start;
        Self {
            world_pos,
            start,
            end,
            width: diff.x,
            height: diff.y,
            size: diff.x * diff.y,
        }
    }

    pub fn empty() -> Self {
        return Self::new(Point::zero(), Point::zero(), Point::zero());
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
    pub delta: u64,
    pub omega: f32,
}

pub struct WorkerInterface {
    pub sender: Sender<WorkerCommand>,
    pub receiver: Receiver<WorkerResponse>,
    pub view: BoardView,
}

impl WorkerInterface {
    pub fn send(&self, message: WorkerCommand) {
        if let Err(err) = self.sender.send(message) {
            println!("Failed to send message to server: {:?}", err);
        }
    }
}
