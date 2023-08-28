mod board;
mod client;
mod render;
mod rsc;
mod util;
mod common;

fn main() {
    pollster::block_on(client::Client::run());
}
