use std::{
    sync::mpsc::{Receiver, Sender},
    time::{Duration, Instant},
};

use rayon::{
    prelude::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

use crate::{
    message::{CameraView, ClientMessage, WorldMessage},
    rsc::{UPDATE_TIME, UPS},
    sync::{BoardViewInfo, BoardViewLock},
    util::{point::Point, timer::Timer},
};

use super::{board::Board, swap_buffer::SwapBuffer};

pub struct World {
    pub board: Board,
    pub board_view: BoardViewLock,
    pub slice: BoardSlice,
    pub slice_change: bool,
    pub update_time: Duration,
    pub paused: bool,
    pub step: bool,
    pub client_ready: bool,
    pub timer: Timer,
    pub sender: Sender<WorldMessage>,
    pub receiver: Receiver<ClientMessage>,
}

impl World {
    pub fn new(
        board_view: BoardViewLock,
        sender: Sender<WorldMessage>,
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
            board_view,
            slice: BoardSlice::default(),
            slice_change: false,
            update_time: UPDATE_TIME,
            paused: true,
            step: false,
            timer: Timer::new(Duration::from_secs(1), UPS as usize),
            sender,
            receiver,
            client_ready: true,
        }
    }

    pub fn run(&mut self) {
        let mut last_update = Instant::now();
        loop {
            let now = Instant::now();
            let udelta = now - last_update;
            if udelta > self.update_time {
                last_update = now;
                self.receive_messages();
                if !self.paused || self.step {
                    self.step = false;

                    self.timer.start();
                    self.board.update();
                    self.timer.stop();
                }
                if (self.slice_change || self.board.dirty) && self.client_ready {
                    self.sync_board();
                }
            }
        }
    }

    fn receive_messages(&mut self) {
        for msg in self.receiver.try_iter() {
            match msg {
                ClientMessage::Swap(pos1, pos2) => {
                    self.board.swap(pos1, pos2);
                }
                ClientMessage::AddEnergy(pos) => {
                    let i = pos.index(self.board.width);
                    self.board.energy.set(i, self.board.energy.get(i) + 10.0);
                    self.board.dirty = true;
                }
                ClientMessage::Pause(set) => self.paused = set,
                ClientMessage::Step() => self.step = true,
                ClientMessage::CameraUpdate(view) => {
                    let new = self.calc_board_slice(view);
                    self.slice_change = self.slice != new;
                    self.slice = new;
                }
                ClientMessage::RenderFinished() => self.client_ready = true,
            }
        }
    }

    fn sync_board(&mut self) {
        let board = &mut self.board;
        let slice = &mut self.slice;
        board.dirty = false;
        self.slice_change = false;

        let mut view = self
            .board_view
            .lock()
            .expect("Failed to get tile view lock");

        copy_swap_buf(&mut view.connex_numbers, &board.connex_numbers, &slice);
        copy_swap_buf(&mut view.stability, &board.stability, &slice);
        copy_swap_buf(&mut view.reactivity, &board.reactivity, &slice);
        copy_swap_buf(&mut view.energy, &board.energy, &slice);
        copy_swap_buf(&mut view.alpha, &board.alpha, &slice);
        copy_swap_buf(&mut view.beta, &board.beta, &slice);
        copy_swap_buf(&mut view.gamma, &board.gamma, &slice);
        copy_swap_buf(&mut view.delta, &board.delta, &slice);
        copy_swap_buf(&mut view.omega, &board.omega, &slice);

        let slice_start: Point<f32> = self.slice.start.into();
        view.info = BoardViewInfo {
            render_info: crate::render::tile::data::RenderViewInfo {
                pos: self.board.pos + slice_start,
                slice: self.slice.clone(),
                dirty: true,
            },
            total_energy: self.board.total_energy,
            time_taken: self.timer.avg(),
        }
    }

    fn calc_board_slice(&self, view: CameraView) -> BoardSlice {
        let corner = Point::new(self.board.width, self.board.height);
        // get camera position relative to board
        let cam_rel_pos: Point<i32> = (view.pos - self.board.pos).into();
        // calculate chunk size based on max camera dimension
        let chunk_align = (view.width.max(view.height) as u32).max(1).ilog2();
        let chunk_size = 2i32.pow(chunk_align);
        let chunk_mask = !(chunk_size - 1);
        // align with chunks and add an extra chunk in each direction
        // s = start, e = end
        let aligned_start = (cam_rel_pos & chunk_mask) - 1 * chunk_size;
        let aligned_end = (cam_rel_pos & chunk_mask) + 2 * chunk_size;
        // clamp to board dimensions
        let bounded_start = aligned_start.clamp_usize(corner);
        let bounded_end = aligned_end.clamp_usize(corner);

        BoardSlice::new(bounded_start, bounded_end)
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

impl BoardSlice {
    pub fn new(start: Point<usize>, end: Point<usize>) -> Self {
        let diff = end - start;
        Self {
            start,
            end,
            width: diff.x,
            height: diff.y,
            size: diff.x * diff.y,
        }
    }
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
