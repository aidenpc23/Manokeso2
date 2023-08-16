use std::{
    sync::{mpsc::Receiver, Arc, RwLock},
    time::{Duration, Instant},
};

use rayon::{
    prelude::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

use crate::{
    board_view::{BoardView, ClientView},
    message::ClientMessage,
    rsc::{UPDATE_TIME, UPS},
    util::{point::Point, timer::Timer},
};

use super::{swap_buffer::SwapBuffer, Board};

pub struct Server {
    pub board: Board,
    pub client_view: Arc<RwLock<ClientView>>,
    pub board_view: Arc<RwLock<BoardView>>,
    pub slice: BoardSlice,
    pub update_time: Duration,
    pub paused: bool,
    pub timer: Timer,
    pub receiver: Receiver<ClientMessage>,
}

impl Server {
    pub fn new(
        client_view: &Arc<RwLock<ClientView>>,
        board_view: &Arc<RwLock<BoardView>>,
        receiver: Receiver<ClientMessage>,
    ) -> Self {
        let width = 1000;
        let height = 1000;
        let board = Board::new(
            Point::new(-(width as f32) / 2.0, -(height as f32) / 2.0),
            width,
            height,
        );
        Self {
            board,
            client_view: client_view.clone(),
            board_view: board_view.clone(),
            slice: BoardSlice::default(),
            update_time: UPDATE_TIME,
            paused: true,
            timer: Timer::new(UPS as usize),
            receiver,
        }
    }

    pub fn run(&mut self) {
        let mut last_update = Instant::now();
        loop {
            let now = Instant::now();
            let udelta = now - last_update;
            if udelta > self.update_time {
                last_update = now;
                for msg in self.receiver.try_iter() {
                    match msg {
                        ClientMessage::Swap(pos1, pos2) => {
                            let pos1 = pos1 + self.slice.start;
                            let pos2 = pos2 + self.slice.start;
                            self.board.player_swap(pos1, pos2);
                        }
                        ClientMessage::AddEnergy(pos) => {
                            let i = (pos + self.slice.start).index(self.board.width);
                            self.board
                                .energy
                                .god_set(i, self.board.energy.god_get(i) + 10.0);
                            self.board.dirty = true;
                        }
                    }
                }
                if !self.paused {
                    self.timer.start();
                    self.board.update();
                    self.timer.stop();
                }
                self.sync();
            }
        }
    }

    fn sync(&mut self) {
        let client = self
            .client_view
            .read()
            .expect("Failed to get tile view lock");
        self.paused = client.paused;
        let slice = self.calc_board_slice(&client);
        drop(client);

        if slice != self.slice || self.board.dirty {
            self.slice = slice;
            self.board.dirty = false;

            let mut view = self
                .board_view
                .write()
                .expect("Failed to get tile view lock");

            copy_swap_buf(&mut view.connex_numbers, &self.board.connex_numbers, &slice);
            copy_swap_buf(&mut view.stability, &self.board.stability, &slice);
            copy_swap_buf(&mut view.reactivity, &self.board.reactivity, &slice);
            copy_swap_buf(&mut view.energy, &self.board.energy, &slice);
            copy_swap_buf(&mut view.alpha, &self.board.alpha, &slice);
            copy_swap_buf(&mut view.beta, &self.board.beta, &slice);
            copy_swap_buf(&mut view.gamma, &self.board.gamma, &slice);
            copy_swap_buf(&mut view.delta, &self.board.delta, &slice);
            copy_swap_buf(&mut view.omega, &self.board.omega, &slice);

            view.pos = self.board.pos + slice.start.into();
            view.slice = slice.clone();
            view.total_energy = self.board.total_energy;
            view.dirty = true;
            view.time_taken = self.timer.avg();
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
            start: Point { x: xs, y: ys },
            end: Point { x: xe, y: ye },
            width,
            height,
            size,
        }
    }
}

#[derive(PartialEq, Default, Clone, Copy)]
pub struct BoardSlice {
    pub start: Point<usize>,
    pub end: Point<usize>,
    pub width: usize,
    pub height: usize,
    pub size: usize,
}

fn copy_swap_buf<T: Send + Sync + Copy>(dest: &mut Vec<T>, sb: &SwapBuffer<T>, slice: &BoardSlice) {
    if dest.len() != slice.size {
        *dest = Vec::with_capacity(slice.size);
        unsafe { dest.set_len(slice.size) }
    }
    dest.par_chunks_exact_mut(slice.width)
        .zip(sb.par_rows(slice.start.y, slice.end.y))
        .for_each(|(data, row)| {
            data.copy_from_slice(&row[slice.start.x..slice.end.x]);
        });
}
