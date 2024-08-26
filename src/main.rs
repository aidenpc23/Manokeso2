use client::ClientApp;
use winit::event_loop::EventLoop;

mod board;
mod client;
mod render;
mod rsc;
mod util;
mod common;

fn main() {
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    event_loop
        .run_app(&mut ClientApp::new())
        .expect("Failed to run event loop");
}
