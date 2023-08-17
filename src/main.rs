mod board_view;
mod client;
mod message;
mod render;
mod rsc;
mod sync;
mod util;
mod world;

fn main() {
    pollster::block_on(client::run());
}
