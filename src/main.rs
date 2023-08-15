use std::time;

use config::Config;
use handle_input::handle_input;
use input::Input;
use state::GameState;
use tile_view::ClientView;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};

mod camera;
mod config;
mod handle_input;
mod input;
mod keybinds;
mod render;
mod rsc;
mod state;
mod tile_view;
mod timer;
mod util;
mod world;

use render::Renderer;
use world::Server;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    // Setup
    env_logger::init();
    let mut state = GameState::new(Config::load());

    let event_loop = EventLoop::new();
    let mut renderer = Renderer::new(&event_loop, &state.camera).await;
    renderer.window.set_visible(true);

    let mut last_update = time::Instant::now();
    let mut last_frame = time::Instant::now();
    let mut input = Input::new();
    let mut resized = false;

    let client_view = ClientView::new();
    let mut server = Server::new();

    // Game loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, window_id } if window_id == renderer.window.id() => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(_) => resized = true,
                    _ => input.update(event),
                }
            }
            Event::RedrawRequested(_) => renderer.render(&state, false),
            Event::MainEventsCleared => {
                let now = time::Instant::now();
                let fdelta = now - last_frame;
                if fdelta > state.frame_time {
                    last_frame = now;

                    if handle_input(&fdelta, &input, &mut state, &renderer) {
                        *control_flow = ControlFlow::Exit;
                    }
                    input.end();

                    state.timers.render.start();
                    renderer.render(&state, resized);
                    state.timers.render.end();

                    resized = false;
                }
            }
            _ => {}
        }
    });
}
