mod board_view;
mod sync;
mod message;
mod render;
mod rsc;
mod util;
mod world;
mod client;

fn main() {
    pollster::block_on(client::run());
}

