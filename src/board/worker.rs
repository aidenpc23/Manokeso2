use std::time::{Duration, Instant};

use crate::{
    client::ClientState,
    common::{
        interface::ClientInterface,
        message::{CameraView, TileChange, WorkerCommand, WorkerResponse},
        save::{load, save},
    },
    rsc::{CONNEX_NUMBER_RANGE, REACTIVITY_RANGE, STABILITY_RANGE, UPDATE_TIME},
    util::{math::SaturatingAdd, point::Point},
};

use super::{board::Board, instance::BoardInstance};

pub struct BoardWorker {
    pub board: BoardInstance,
    pub update_time: Duration,
    pub paused: bool,
    pub step: bool,
    pub client: ClientInterface,
    pub cam_view: CameraView,
}

impl BoardWorker {
    pub fn new(client: ClientInterface) -> Self {
        let width = 708;
        let height = 354;
        let board = Board::new(
            Point::new(-(width as f32) / 2.0, -(height as f32) / 2.0),
            width,
            height,
        );
        Self {
            board: BoardInstance::new(board),
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

                    self.board.update();
                }
                self.sync_board();
            }
        }
        println!("exiting...");
    }

    fn receive_messages(&mut self, target: &mut Instant) -> bool {
        let mut new_view = false;
        let mut msgs: Vec<WorkerCommand> = Vec::new();
        if self.paused {
            let start = std::time::Instant::now();
            msgs.push(self.client.receiver.recv().expect("client died??"));
            *target += std::time::Instant::now() - start;
        }
        msgs.extend(self.client.receiver.try_iter());
        for msg in msgs {
            match msg {
                WorkerCommand::Swap(pos1, pos2, creative) => {
                    if creative || self.board.read().player_can_swap(pos1, pos2) {
                        self.board.write().swap(pos1, pos2);
                    }
                }
                WorkerCommand::Save(name, state) => {
                    if let Err(err) = save(&name, &(&self.board.read(), state)) {
                        println!("{:?}", err);
                    }
                }
                WorkerCommand::Load(name) => match load::<(Board, ClientState)>(&name) {
                    Ok(data) => {
                        self.board = BoardInstance::new(data.0);
                        self.paused = true;
                        new_view = true;
                        self.client.send(WorkerResponse::Loaded(data.1));
                    }
                    Err(err) => println!("{:?}", err),
                },
                WorkerCommand::ChangeTile(pos, change) => {
                    let i = pos.index(self.board.read().width);
                    let bufs = &mut self.board.write().bufs;
                    match change {
                        TileChange::ConnexNumber(amt) => {
                            bufs.connex_numbers.r[i] = bufs.connex_numbers.r[i]
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
                WorkerCommand::ViewSwap(view) => self.client.view = Some(view),
                WorkerCommand::Exit() => return true,
            }
        }
        if new_view {
            self.board.update_view(&self.cam_view);
        }
        false
    }

    fn sync_board(&mut self) {
        if let Some(mut view) = self.client.view.take() {
            if self.board.sync(&mut view) {
                self.client.send(WorkerResponse::ViewSwap(view));
            }
        }
    }
}
