use crate::{util::point::Point, sync::BoardView};

#[derive(Debug)]
pub struct CameraView {
    pub pos: Point<f32>,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug)]
pub enum ClientMessage {
    CameraUpdate(CameraView),
    ChangeTile(Point<usize>, TileChange),
    Swap(Point<usize>, Point<usize>),
    Pause(bool),
    Step(),
    ViewSwap(BoardView),
}

#[derive(Debug)]
pub enum TileChange {
    ConnexNumber(i32),
    Stability(f32),
    Energy(f32),
    Reactivity(f32),
    Omega(f32)
}

pub enum WorldMessage {
    ViewSwap(BoardView)
}
