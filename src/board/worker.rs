use std::time::{Duration, Instant};

use crate::{
    client::ClientState,
    common::{
        interface::ClientInterface,
        message::{CameraView, TileChange, WorkerCommand, WorkerResponse},
        save::{load, save}, view::BoardView,
    },
    rsc::{CONNEX_NUMBER_RANGE, REACTIVITY_RANGE, STABILITY_RANGE, UPDATE_TIME},
    util::math::SaturatingAdd,
};

use super::{board::Board, instance::BoardInstance};

pub struct BoardWorker {
    pub boards: Vec<BoardInstance>,
    pub update_time: Duration,
    pub paused: bool,
    pub step: bool,
    pub client: ClientInterface,
    pub cam_view: CameraView,
}

impl BoardWorker {
    pub fn new(client: ClientInterface) -> Self {
        Self {
            boards: Vec::new(),
            update_time: UPDATE_TIME,
            paused: true,
            step: false,
            client,
            cam_view: CameraView::empty(),
        }
    }

    pub fn run(&mut self) {
        let mut target = Instant::now();
        loop {
            let now = Instant::now();
            if now >= target {
                target += self.update_time;
                if self.receive_messages(&mut target) {
                    break;
                }
                if !self.paused || self.step {
                    self.step = false;

                    for board in &mut self.boards {
                        board.update();
                    }
                }
                self.sync_board();
            }
        }
        println!("exiting...");
    }

    fn receive_messages(&mut self, target: &mut Instant) -> bool {
        let mut new_view = false;
        let mut msgs: Vec<_> = Vec::new();
        if self.paused {
            let start = std::time::Instant::now();
            msgs.push(self.client.receiver.recv().expect("client died??"));
            *target += std::time::Instant::now() - start;
        }
        msgs.extend(self.client.receiver.try_iter());
        for msg in msgs {
            match msg {
                WorkerCommand::Swap(t1, t2, creative) => {
                    let b1 = &self.boards[t1.board_id];
                    let b2 = &self.boards[t2.board_id];
                    if creative
                        || (b1.read().player_can_move(t1.pos) && b2.read().player_can_move(t2.pos))
                    {
                        let i1 = t1.pos.index(b1.read().width);
                        let i2 = t2.pos.index(b2.read().width);
                        let a1 = b1.read().bufs.get_tile(i1);
                        let a2 = b2.read().bufs.get_tile(i2);
                        self.boards[t1.board_id].write().bufs.set_tile(i1, a2);
                        self.boards[t2.board_id].write().bufs.set_tile(i2, a1);
                    }
                }
                WorkerCommand::Save(name, state) => {
                    let boards: Vec<_> = self.boards.iter().map(|b| b.read()).collect();
                    if let Err(err) = save(&name, &(&boards, state)) {
                        println!("{:?}", err);
                    }
                }
                WorkerCommand::Load(name) => match load::<(Vec<Board>, ClientState)>(&name) {
                    Ok(mut data) => {
                        self.boards = data.0.drain(..).map(|b| BoardInstance::new(b)).collect();
                        self.paused = true;
                        new_view = true;
                        self.client.send(WorkerResponse::Loaded(data.1));
                    }
                    Err(err) => println!("{:?}", err),
                },
                WorkerCommand::ChangeTile(tile, change) => {
                    let board = &mut self.boards[tile.board_id as usize];
                    let i = tile.pos.index(board.read().width);
                    let bufs = &mut board.write().bufs;
                    match change {
                        TileChange::ConnexNumber(amt) => {
                            bufs.connex_number.r[i] = bufs.connex_number.r[i]
                                .sat_add(amt)
                                .clamp(CONNEX_NUMBER_RANGE[0], CONNEX_NUMBER_RANGE[1]);
                        }
                        TileChange::Stability(amt) => {
                            bufs.stability.r[i] = (bufs.stability.r[i] + amt)
                                .clamp(STABILITY_RANGE[0], STABILITY_RANGE[1]);
                        }
                        TileChange::Energy(amt) => {
                            bufs.energy.r[i] += amt;
                            bufs.energy.r[i] = bufs.energy.r[i].max(0.0);
                        }
                        TileChange::Reactivity(amt) => {
                            bufs.reactivity.r[i] = (bufs.reactivity.r[i] + amt)
                                .clamp(REACTIVITY_RANGE[0], REACTIVITY_RANGE[1]);
                            if bufs.reactivity.r[i].abs() < 0.001 {
                                bufs.reactivity.r[i] = 0.0;
                            }
                        }
                        TileChange::Delta(amt) => {
                            bufs.delta.r[i] = bufs.delta.r[i].sat_add(amt);
                        }
                    }
                }
                WorkerCommand::Pause(set) => self.paused = set,
                WorkerCommand::Step() => self.step = true,
                WorkerCommand::CameraUpdate(view) => {
                    self.cam_view = view;
                    new_view = true;
                }
                WorkerCommand::ViewsSwapped(view) => self.client.views = Some(view),
                WorkerCommand::CreateBoard(settings) => {
                    self.boards.push(BoardInstance::new(Board::new(settings)));
                }
                WorkerCommand::Exit() => return true,
            }
        }
        if new_view {
            for board in &mut self.boards {
                board.update_view(&self.cam_view);
            }
        }
        false
    }

    fn sync_board(&mut self) {
        if let Some(mut views) = self.client.views.take() {
            for (i, board) in self.boards.iter_mut().enumerate() {
                if i == views.len() {
                    views.push(BoardView::empty());
                }
                board.sync(&mut views[i]);
            }
            self.client.send(WorkerResponse::ViewsUpdated(views));
        }
    }
}
