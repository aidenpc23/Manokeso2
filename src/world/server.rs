use std::{
    sync::RwLock,
    time::{Duration, Instant},
};

use rayon::{
    prelude::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

use crate::{
    rsc::UPDATE_TIME,
    tile_view::{BoardView, ClientView},
    util::point::Point,
};

use super::{swap_buffer::SwapBuffer, Board};

pub struct Server {
    pub board: Board,
    pub client_view: RwLock<ClientView>,
    pub board_view: RwLock<BoardView>,
    pub update_time: Duration,
    pub paused: bool,
}

impl Server {
    pub fn new(client_view: RwLock<ClientView>, board_view: RwLock<BoardView>) -> Self {
        let width = 1000;
        let height = 1000;
        let board = Board::new(
            Point::new(-(width as f32) / 2.0, -(height as f32) / 2.0),
            width,
            height,
        );
        Self {
            board,
            client_view,
            board_view,
            update_time: UPDATE_TIME,
            paused: true,
        }
    }

    pub async fn run(&mut self) {
        let mut last_update = Instant::now();
        loop {
            let now = Instant::now();
            let udelta = now - last_update;
            if udelta > self.update_time {
                last_update = now;
                if !self.paused {
                    self.board.update();
                    let client = self
                        .client_view
                        .read()
                        .expect("Failed to get tile view lock");
                    let slice = self.calc_board_slice(&client);
                    drop(client);

                    let mut view = self
                        .board_view
                        .write()
                        .expect("Failed to get tile view lock");
                    copy_swap_buf(&mut view.connex_numbers, &self.board.connex_numbers, &slice);
                    copy_swap_buf(&mut view.stability, &self.board.stability, &slice);
                    copy_swap_buf(&mut view.reactivity, &self.board.reactivity, &slice);
                    copy_swap_buf(&mut view.energy, &self.board.energy, &slice);
                    view.pos = self.board.pos + Point::new(slice.xs as f32, slice.ys as f32);
                    view.width = slice.width;
                    view.height = slice.height;
                    view.total_energy = self.board.total_energy;
                    view.dirty = self.board.dirty;
                }
            }
        }
    }

    fn calc_board_slice(&self, view: &ClientView) -> BoardSlice {
        // get positions in the world
        let b = self.board.pos;
        let bw = self.board.width;
        let bh = self.board.height;
        let (cw, ch) = (view.width, view.height);
        // get camera position relative to board
        let x = (view.pos.x - b.x + 0.5) as i32;
        let y = (view.pos.y - b.y + 0.5) as i32;
        // calculate chunk size based on max camera dimension
        let chunk_align = (cw.max(ch) as u32).max(1).ilog2();
        let chunk_size = 2i32.pow(chunk_align);
        let chunk_mask = !(chunk_size - 1);
        // align with chunks and add an extra chunk in each direction
        // s = start, e = end
        let xs = (x & chunk_mask) - 1 * chunk_size;
        let ys = (y & chunk_mask) - 1 * chunk_size;
        let xe = (x & chunk_mask) + 2 * chunk_size;
        let ye = (y & chunk_mask) + 2 * chunk_size;
        // cut off values for bounds
        let xs = (xs.max(0) as usize).min(bw);
        let ys = (ys.max(0) as usize).min(bh);
        let xe = (xe.max(0) as usize).min(bw);
        let ye = (ye.max(0) as usize).min(bh);

        let width = xe - xs;
        let height = ye - ys;
        let size = width * height;

        BoardSlice {
            xs,
            xe,
            ys,
            ye,
            width,
            height,
            size,
        }
    }
}

pub struct BoardSlice {
    pub xs: usize,
    pub xe: usize,
    pub ys: usize,
    pub ye: usize,
    pub width: usize,
    pub height: usize,
    pub size: usize,
}

fn copy_swap_buf<T: Send + Sync + Copy>(dest: &mut Vec<T>, sb: &SwapBuffer<T>, slice: &BoardSlice) {
    if dest.len() != slice.size {
        *dest = Vec::with_capacity(slice.size);
    }
    dest.par_chunks_exact_mut(slice.width)
        .zip(sb.par_rows(slice.ys, slice.ye))
        .for_each(|(data, row)| {
            data.copy_from_slice(&row[slice.xs..slice.xe]);
        });
}
