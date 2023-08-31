use std::time::Duration;

use crate::{
    common::{
        message::CameraView,
        view::{BoardSlice, BoardView},
    },
    rsc::{CHUNK_VIEW_RADIUS, MIN_CHUNK_SIZE, UPS},
    util::{point::Point, timer::Timer},
};

use super::Board;

pub struct BoardInstance {
    board: Board,
    board_changed: bool,
    slice: BoardSlice,
    slice_changed: bool,
    timer: Timer,
}

impl BoardInstance {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            board_changed: true,
            slice: BoardSlice::empty(),
            slice_changed: false,
            timer: Timer::new(Duration::from_secs(1), UPS as usize),
        }
    }
    pub fn read(&self) -> &Board {
        &self.board
    }
    pub fn update(&mut self) {
        self.timer.start();
        self.write().update();
        self.timer.stop();
    }
    pub fn write(&mut self) -> &mut Board {
        self.board_changed = true;
        &mut self.board
    }
    pub fn sync(&mut self, view: &mut BoardView) -> bool {
        if self.board_changed || self.slice_changed {
            self.board_changed = false;
            self.board.bufs.copy_to_view(&mut view.bufs, &self.slice);
            view.slice = self.slice.clone();
            view.total_energy = self.board.total_energy;
            view.board_pos = self.board.pos;
            view.time_taken = self.timer.avg();
            true
        } else {
            false
        }
    }
    pub fn update_view(&mut self, view: &CameraView) {
        let new = calc_board_slice(&self.board, view);
        self.slice_changed |= self.slice != new;
        self.slice = new;
    }
}

fn calc_board_slice(board: &Board, view: &CameraView) -> BoardSlice {
    let corner = Point::new(board.width, board.height);
    // get camera position relative to board
    let cam_rel_pos: Point<i32> = (view.pos - board.pos).into();
    // calculate chunk size based on max camera dimension
    let chunk_align = (view.width.max(view.height) as u32).max(1).ilog2();
    let chunk_size = 2i32.pow(chunk_align).max(MIN_CHUNK_SIZE);
    let chunk_mask = !(chunk_size - 1);
    // align with chunks and add an extra chunk in each direction
    // s = start, e = end
    let aligned_start = (cam_rel_pos & chunk_mask) - CHUNK_VIEW_RADIUS * chunk_size;
    let aligned_end = (cam_rel_pos & chunk_mask) + (CHUNK_VIEW_RADIUS + 1) * chunk_size;
    // clamp to board dimensions
    let bounded_start = aligned_start.clamp_usize(corner);
    let bounded_end = aligned_end.max(aligned_start).clamp_usize(corner);

    let start_f32: Point<f32> = bounded_start.into();

    BoardSlice::new(board.pos + start_f32, bounded_start, bounded_end)
}
