use crate::{util::point::Point, view::BoardView};

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
    Swap(Point<usize>, Point<usize>, bool),
    Pause(bool),
    Save(),
    Load(),
    Step(),
    ViewSwap(BoardView),
}

#[derive(Debug)]
pub enum TileChange {
    ConnexNumber(i32),
    Stability(f32),
    Energy(f32),
    Reactivity(f32),
    Delta(i32)
}

pub enum WorldMessage {
    ViewSwap(BoardView)
}
