use std::time;

use input::Input;
use rsc::FRAME_TIME;
use state::GameState;
use update::update;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};

mod camera;
mod input;
mod render;
mod rsc;
mod state;
mod update;
mod world;
mod timer;

use render::Renderer;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    // Setup logger, event loop, and window
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut state = GameState::new();
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
                    _ => inputs.update(event),
                }
            }
            Event::RedrawRequested(_) => renderer.render(),
            Event::MainEventsCleared => {
                let now = time::Instant::now();
                // handle update
                let delta = now - last_update;
                last_update = now;
                if update(&delta, &inputs, &mut state) {
                    *control_flow = ControlFlow::Exit;
                }
                inputs.end();
                // render if it's time
                let delta = now - last_frame;
                if delta > FRAME_TIME {
                    last_frame = now;
                    let ustart = time::Instant::now();
                    state.board.update(&delta);
                    let fstart = time::Instant::now();
                    renderer.update(&state, resized);
                    resized = false;
                    renderer.render();
                    let fend = time::Instant::now();
                    state.timers.update.push(fstart - ustart);
                    state.timers.frame.push(fend - fstart);
                }
            }
            _ => {}
        }
    });
}
