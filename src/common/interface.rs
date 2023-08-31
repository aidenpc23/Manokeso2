use crate::{common::message::{WorkerCommand, WorkerResponse}, board::BoardSettings};
use std::sync::mpsc::{channel, Receiver, Sender};

use super::view::BoardView;

pub struct WorkerInterface {
    pub sender: Sender<WorkerCommand>,
    pub receiver: Receiver<WorkerResponse>,
    views: Vec<BoardView>,
}

impl WorkerInterface {
    pub fn send(&self, message: WorkerCommand) {
        if let Err(err) = self.sender.send(message) {
            println!("Failed to send message to server: {:?}", err);
        }
    }
    pub fn swap(&mut self, views: &mut Vec<BoardView>) {
        std::mem::swap(&mut self.views, views);
    }
    pub fn get(&self, index: usize) -> Option<&BoardView> {
        self.views.get(index)
    }
    pub fn views(&self) -> std::slice::Iter<'_, BoardView> {
        self.views.iter()
    }
}

pub struct ClientInterface {
    pub sender: Sender<WorkerResponse>,
    pub receiver: Receiver<WorkerCommand>,
    pub views: Option<Vec<BoardView>>,
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
            views: Vec::new(),
        },
        ClientInterface {
            sender: s2,
            receiver: r1,
            views: Some(Vec::new()),
        },
    )
}
