use crate::{board::BoardViewBufs, util::point::Point};
use std::time::Duration;

pub type BoardId = usize;

#[derive(Debug)]
pub struct BoardView {
    pub board_pos: Point<f32>,
    pub bufs: BoardViewBufs,
    pub slice: BoardSlice,
    pub total_energy: f32,
    pub time_taken: Duration,
}

impl BoardView {
    pub fn empty() -> Self {
        Self {
            board_pos: Point::zero(),
            bufs: BoardViewBufs::empty(),
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
