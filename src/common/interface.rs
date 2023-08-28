use crate::common::message::{WorkerCommand, WorkerResponse};
use std::sync::mpsc::{channel, Receiver, Sender};

use super::view::BoardView;

pub struct WorkerInterface {
    pub sender: Sender<WorkerCommand>,
    pub receiver: Receiver<WorkerResponse>,
    pub view: BoardView,
}

impl WorkerInterface {
    pub fn send(&self, message: WorkerCommand) {
        if let Err(err) = self.sender.send(message) {
            println!("Failed to send message to server: {:?}", err);
        }
    }
}

pub struct ClientInterface {
    pub sender: Sender<WorkerResponse>,
    pub receiver: Receiver<WorkerCommand>,
    pub view: Option<BoardView>,
}

impl ClientInterface {
    pub fn send(&self, response: WorkerResponse) {
        if let Err(err) = self.sender.send(response) {
            println!("Failed to send message to server: {:?}", err);
        }
    }
}

pub fn interface_pair() -> (WorkerInterface, ClientInterface) {
    let (s1, r1) = channel();
    let (s2, r2) = channel();
    (
        WorkerInterface {
            sender: s1,
            receiver: r2,
            view: BoardView::empty(),
        },
        ClientInterface {
            sender: s2,
            receiver: r1,
            view: Some(BoardView::empty()),
        },
    )
}
