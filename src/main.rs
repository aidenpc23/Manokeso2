mod board;
mod client;
mod message;
mod render;
mod rsc;
mod util;
mod view;

fn main() {
    pollster::block_on(client::Client::run());
}
