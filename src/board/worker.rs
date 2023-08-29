use std::time::{Duration, Instant};

use crate::{
    client::ClientState,
    common::{
        interface::ClientInterface,
        message::{CameraView, TileChange, WorkerCommand, WorkerResponse},
        save::{load, save},
        view::BoardSlice,
    },
    rsc::{
        CHUNK_VIEW_RADIUS, CONNEX_NUMBER_RANGE, MIN_CHUNK_SIZE, REACTIVITY_RANGE, STABILITY_RANGE,
        UPDATE_TIME, UPS,
    },
    util::{math::SaturatingAdd, point::Point, timer::Timer},
};

use super::board::Board;

pub struct BoardWorker {
    pub board: Board,
    pub dirty: bool,
    pub slice: BoardSlice,
    pub slice_change: bool,
    pub update_time: Duration,
    pub paused: bool,
    pub step: bool,
    pub timer: Timer,
    pub client: ClientInterface,
    pub cam_view: CameraView,
}

impl BoardWorker {
    pub fn new(client: ClientInterface) -> Self {
        let width = 708;
        let height = 708;
        let board = Board::new(
            Point::new(-(width as f32) / 2.0, -(height as f32) / 2.0),
            width,
            height,
        );
        Self {
            board,
            slice: BoardSlice::empty(),
            slice_change: false,
            update_time: UPDATE_TIME,
            paused: true,
            step: false,
            client,
            timer: Timer::new(Duration::from_secs(1), UPS as usize),
            cam_view: CameraView::empty(),
            dirty: false,
        }
    }

    pub fn run(&mut self) {
        let mut target = Instant::now();
        loop {
            let now = Instant::now();
            if now > target {
                target += self.update_time;
                if self.receive_messages() {
                    break;
                }
                if !self.paused || self.step {
                    self.step = false;

                    self.timer.start();
                    self.board.update();
                    self.dirty = true;
                    self.timer.stop();
                }
                if self.slice_change || self.dirty {
                    self.sync_board();
                }
            }
        }
        println!("exiting...");
    }

    fn receive_messages(&mut self) -> bool {
        let mut new_view = false;
        let mut msgs: Vec<WorkerCommand> = Vec::new();
        if self.paused {
            msgs.push(self.client.receiver.recv().expect("client died??"));
        }
        msgs.extend(self.client.receiver.try_iter());
        for msg in msgs {
            match msg {
                WorkerCommand::Swap(pos1, pos2, creative) => {
                    if creative || self.board.player_can_swap(pos1, pos2) {
                        self.board.swap(pos1, pos2);
                        self.dirty = true;
                    }
                }
                WorkerCommand::Save(name, state) => {
                    if let Err(err) = save(&name, &(&self.board, state)) {
                        println!("{:?}", err);
                    }
                }
                WorkerCommand::Load(name) => match load::<(Board, ClientState)>(&name) {
                    Ok(data) => {
                        self.board = data.0;
                        self.dirty = true;
                        self.paused = true;
                        new_view = true;
                        self.client.send(WorkerResponse::Loaded(data.1));
                    }
                    Err(err) => println!("{:?}", err),
                },
                WorkerCommand::ChangeTile(pos, change) => {
                    let i = pos.index(self.board.width);
                    match change {
                        TileChange::ConnexNumber(amt) => {
                            self.board.bufs.connex_numbers.r[i] = self.board.bufs.connex_numbers.r
                                [i]
                                .sat_add(amt)
                                .clamp(CONNEX_NUMBER_RANGE[0], CONNEX_NUMBER_RANGE[1]);
                        }
                        TileChange::Stability(amt) => {
                            self.board.bufs.stability.r[i] = (self.board.bufs.stability.r[i] + amt)
                                .clamp(STABILITY_RANGE[0], STABILITY_RANGE[1]);
                        }
                        TileChange::Energy(amt) => {
                            self.board.bufs.energy.r[i] += amt;
                            self.board.bufs.energy.r[i] = self.board.bufs.energy.r[i].max(0.0);
                        }
                        TileChange::Reactivity(amt) => {
                            self.board.bufs.reactivity.r[i] = (self.board.bufs.reactivity.r[i]
                                + amt)
                                .clamp(REACTIVITY_RANGE[0], REACTIVITY_RANGE[1]);
                            if self.board.bufs.reactivity.r[i].abs() < 0.001 {
                                self.board.bufs.reactivity.r[i] = 0.0;
                            }
                        }
                        TileChange::Delta(amt) => {
                            self.board.bufs.delta.r[i] = self.board.bufs.delta.r[i].sat_add(amt);
                        }
                    }
                    self.dirty = true;
                }
                WorkerCommand::Pause(set) => self.paused = set,
                WorkerCommand::Step() => self.step = true,
                WorkerCommand::CameraUpdate(view) => {
                    self.cam_view = view;
                    new_view = true;
                }
                WorkerCommand::ViewSwap(view) => self.client.view = Some(view),
                WorkerCommand::Exit() => return true
            }
        }
        if new_view {
            let new = calc_board_slice(&self.board, &self.cam_view);
            self.slice_change |= self.slice != new;
            self.slice = new;
        }
        false
    }

    fn sync_board(&mut self) {
        if let Some(mut view) = self.client.view.take() {
            let slice = &mut self.slice;
            self.dirty = false;
            self.slice_change = false;

            self.board.bufs.copy_to_view(&mut view.bufs, slice);

            view.slice = self.slice.clone();
            view.total_energy = self.board.total_energy;
            view.time_taken = self.timer.avg();
            view.board_pos = self.board.pos;
            self.client.send(WorkerResponse::ViewSwap(view));
        }
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
