use std::time;

use config::Config;
use handle_input::handle_input;
use input::Input;
use state::GameState;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};

mod camera;
mod config;
mod handle_input;
mod input;
mod render;
mod rsc;
mod state;
mod timer;
mod world;
mod keybinds;

use render::Renderer;

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
    let mut inputs = Input::new();
    let mut resized = false;

    // Game loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, window_id } if window_id == renderer.window.id() => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(_) => resized = true,
                    _ => inputs.update(event, &renderer),
                }
            }
            Event::RedrawRequested(_) => renderer.render(),
            Event::MainEventsCleared => {
                let now = time::Instant::now();
                let udelta = now - last_update;
                let fdelta = now - last_frame;
                if udelta > state.update_time {
                    last_update = now;
                    if !state.paused || state.step {
                        state.timers.update.start();
                        state.board.update(&fdelta);
                        state.timers.update.end();
                        state.step = false;
                    }
                }
                if fdelta > state.frame_time {
                    last_frame = now;

                    if handle_input(&fdelta, &inputs, &mut state) {
                        *control_flow = ControlFlow::Exit;
                    }
                    inputs.end();

                    state.timers.render_extract.start();
                    renderer.extract(&state);
                    state.timers.render_extract.end();

                    state.timers.render_write.start();
                    renderer.update(resized);
                    state.timers.render_write.end();

                    state.timers.render_draw.start();
                    renderer.render();
                    state.timers.render_draw.end();

                    resized = false;
                }
            }
            _ => {}
        }
    });
}
