mod client;
mod message;
mod render;
mod rsc;
mod view;
mod util;
mod world;

fn main() {
    pollster::block_on(client::run());
}
