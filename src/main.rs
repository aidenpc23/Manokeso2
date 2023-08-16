mod board_view;
mod camera;
mod config;
mod handle_input;
mod input;
mod keybinds;
mod message;
mod render;
mod rsc;
mod state;
mod util;
mod world;
mod client;

fn main() {
    pollster::block_on(client::run());
}

