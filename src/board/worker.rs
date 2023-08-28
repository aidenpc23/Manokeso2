use std::{
    ops::AddAssign,
    sync::mpsc::{Receiver, Sender},
    time::{Duration, Instant},
};

use rayon::{
    prelude::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

use crate::{
    message::{CameraView, WorkerCommand, TileChange, WorkerResponse},
    rsc::{CONNEX_NUMBER_RANGE, REACTIVITY_RANGE, STABILITY_RANGE, UPDATE_TIME, UPS},
    view::{BoardView, BoardSlice},
    util::{math::SaturatingAdd, point::Point, timer::Timer},
};

use super::{
    board::Board,
    save::{load_game, save_game},
    swap_buffer::SwapBuffer,
};

pub struct BoardWorker {
    pub board: Board,
    pub view: Option<BoardView>,
    pub slice: BoardSlice,
    pub slice_change: bool,
    pub update_time: Duration,
    pub paused: bool,
    pub step: bool,
    pub client_ready: bool,
    pub timer: Timer,
    pub sender: Sender<WorkerResponse>,
    pub receiver: Receiver<WorkerCommand>,
}

impl BoardWorker {
    pub fn new(sender: Sender<WorkerResponse>, receiver: Receiver<WorkerCommand>) -> Self {
        let width = 708;
        let height = 708;
        let board = Board::new(
            Point::new(-(width as f32) / 2.0, -(height as f32) / 2.0),
            width,
            height,
        );
        Self {
            board,
            view: Some(BoardView::empty()),
            slice: BoardSlice::empty(),
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
                if self.slice_change || self.board.dirty {
                    self.sync_board();
                }
            }
        }
    }

    fn receive_messages(&mut self) {
        for msg in self.receiver.try_iter() {
            match msg {
                WorkerCommand::Swap(pos1, pos2, creative) => {
                    if creative || self.board.player_can_swap(pos1, pos2) {
                        self.board.swap(pos1, pos2);
                    }
                }
                WorkerCommand::Save() => {
                    if let Err(err) = save_game("save", &self.board) {
                        println!("{:?}", err);
                    }
                }
                WorkerCommand::Load() => {
                    if let Err(err) = load_game(&mut self.board, "save") {
                        println!("{:?}", err);
                    }
                }
                WorkerCommand::ChangeTile(pos, change) => {
                    let i = pos.index(self.board.width);
                    match change {
                        TileChange::ConnexNumber(amt) => {
                            self.board.connex_numbers.r[i] = self.board.connex_numbers.r[i]
                                .sat_add(amt)
                                .clamp(CONNEX_NUMBER_RANGE[0], CONNEX_NUMBER_RANGE[1]);
                        }
                        TileChange::Stability(amt) => {
                            self.board.stability.r[i] = (self.board.stability.r[i] + amt)
                                .clamp(STABILITY_RANGE[0], STABILITY_RANGE[1]);
                        }
                        TileChange::Energy(amt) => {
                            self.board.energy.r[i] += amt;
                            self.board.energy.r[i] = self.board.energy.r[i].max(0.0);
                        }
                        TileChange::Reactivity(amt) => {
                            self.board.reactivity.r[i] = (self.board.reactivity.r[i] + amt)
                                .clamp(REACTIVITY_RANGE[0], REACTIVITY_RANGE[1]);
                            if self.board.reactivity.r[i].abs() < 0.001 {
                                self.board.reactivity.r[i] = 0.0;
                            }
                        }
                        TileChange::Delta(amt) => {
                            self.board.delta.r[i] = self.board.delta.r[i].sat_add(amt);
                        }
                    }
                    self.board.dirty = true;
                }
                WorkerCommand::Pause(set) => self.paused = set,
                WorkerCommand::Step() => self.step = true,
                WorkerCommand::CameraUpdate(view) => {
                    let new = self.calc_board_slice(view);
                    self.slice_change = self.slice != new;
                    self.slice = new;
                }
                WorkerCommand::ViewSwap(view) => self.view = Some(view),
            }
        }
    }

    fn sync_board(&mut self) {
        if let Some(mut view) = self.view.take() {
            let board = &mut self.board;
            let slice = &mut self.slice;
            board.dirty = false;
            self.slice_change = false;

            copy_swap_buf(&mut view.connex_numbers, &board.connex_numbers, &slice);
            copy_swap_buf(&mut view.stability, &board.stability, &slice);
            copy_swap_buf(&mut view.reactivity, &board.reactivity, &slice);
            copy_swap_buf(&mut view.energy, &board.energy, &slice);
            copy_swap_buf(&mut view.alpha, &board.alpha, &slice);
            copy_swap_buf(&mut view.beta, &board.beta, &slice);
            copy_swap_buf(&mut view.gamma, &board.gamma, &slice);
            copy_swap_buf(&mut view.delta, &board.delta, &slice);
            copy_swap_buf(&mut view.omega, &board.omega, &slice);

            view.slice = self.slice.clone();
            view.total_energy = self.board.total_energy;
            view.time_taken = self.timer.avg();
            view.board_pos = self.board.pos;
            self.sender.send(WorkerResponse::ViewSwap(view)).expect("D:");
        }
    }

    fn calc_board_slice(&self, view: CameraView) -> BoardSlice {
        let corner = Point::new(self.board.width, self.board.height);
        // get camera position relative to board
        let cam_rel_pos: Point<i32> = (view.pos - self.board.pos).into();
        // calculate chunk size based on max camera dimension
        let chunk_align = (view.width.max(view.height) as u32).max(1).ilog2().max(5);
        let chunk_size = 2i32.pow(chunk_align);
        let chunk_mask = !(chunk_size - 1);
        // align with chunks and add an extra chunk in each direction
        // s = start, e = end
        let aligned_start = (cam_rel_pos & chunk_mask) - 1 * chunk_size;
        let aligned_end = (cam_rel_pos & chunk_mask) + 2 * chunk_size;
        // clamp to board dimensions
        let bounded_start = aligned_start.clamp_usize(corner);
        let bounded_end = aligned_end.clamp_usize(corner);

        let start_f32: Point<f32> = bounded_start.into();

        BoardSlice::new(self.board.pos + start_f32, bounded_start, bounded_end)
    }
}

fn copy_swap_buf<T: Send + Sync + Copy + AddAssign>(
    dest: &mut Vec<T>,
    sb: &SwapBuffer<T>,
    slice: &BoardSlice,
) {
    if dest.len() != slice.size {
        *dest = Vec::with_capacity(slice.size);
        unsafe { dest.set_len(slice.size) }
    }
    if slice.size != 0 {
        dest.par_chunks_exact_mut(slice.width)
            .zip(sb.par_rows(slice.start.y, slice.end.y))
            .for_each(|(data, row)| {
                data.copy_from_slice(&row[slice.start.x..slice.end.x]);
            });
    }
}
