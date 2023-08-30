use crate::{client::ClientState, common::view::BoardView, util::point::Point};

#[derive(Debug)]
pub struct CameraView {
    pub pos: Point<f32>,
    pub width: f32,
    pub height: f32,
}

impl CameraView {
    pub fn empty() -> Self {
        Self {
            pos: Point::zero(),
            width: 0.0,
            height: 0.0,
        }
    }
}

#[derive(Debug)]
pub enum WorkerCommand {
    CameraUpdate(CameraView),
    ChangeTile(Point<usize>, TileChange),
    Swap(Point<usize>, Point<usize>, bool),
    Pause(bool),
    Save(String, ClientState),
    Load(String),
    Step(),
    ViewSwap(BoardView),
    Exit(),
}

#[derive(Debug)]
pub enum TileChange {
    ConnexNumber(i32),
    Stability(f32),
    Energy(f32),
    Reactivity(f32),
    Delta(i32),
}

pub enum WorkerResponse {
    ViewSwap(BoardView),
    Loaded(ClientState),
}
