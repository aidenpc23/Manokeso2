use std::sync::mpsc::channel;

use client::Client;
use config::Config;

mod board_view;
mod camera;
mod client;
mod config;
mod handle_input;
mod input;
mod keybinds;
mod message;
mod render;
mod rsc;
mod util;
mod world;

use render::Renderer;
use winit::event_loop::EventLoop;
use world::World;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    // Setup
    let (cs, cr) = channel();

    let event_loop = EventLoop::new();
    let client = Client::new(Config::load(), cs);
    let renderer = Renderer::new(&event_loop).await;

    let bv = client.board_view.clone();

    std::thread::spawn(move || {
        World::new(bv, cr).run();
    });

    client.run(renderer, event_loop);
}
