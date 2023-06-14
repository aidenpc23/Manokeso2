use std::time;

use input::Input;
use rsc::FRAME_TIME;
use state::GameState;
use update::handle_input;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};

mod camera;
mod input;
mod render;
mod rsc;
mod state;
mod timer;
mod update;
mod world;

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
                let delta = now - last_frame;
                if delta > FRAME_TIME {
                    last_frame = now;

                    if handle_input(&delta, &inputs, &mut state) {
                        *control_flow = ControlFlow::Exit;
                    }
                    inputs.end();

                    state.timers.update.start();
                    state.board.update(&delta);
                    state.timers.update.end();

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
