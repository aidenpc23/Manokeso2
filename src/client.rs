use std::{sync::mpsc::channel, time::Instant};

use winit::{event_loop::{EventLoop, ControlFlow}, event::{Event, WindowEvent}};

use crate::{state::ClientState, config::Config, render::Renderer, world::World, input::Input, handle_input::handle_input};

pub async fn run() {
    // Setup
    let (cs, cr) = channel();

    let event_loop = EventLoop::new();
    let mut state = ClientState::new(Config::load(), cs);
    let mut renderer = Renderer::new(&event_loop).await;

    let bv = state.board_view.clone();

    std::thread::spawn(move || {
        World::new(bv, cr).run();
    });

    let mut last_frame = Instant::now();
    let mut input = Input::new();
    let mut resized = false;

    renderer.window.set_visible(true);

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
                let now = Instant::now();
                let fdelta = now - last_frame;
                if fdelta > state.frame_time {
                    last_frame = now;

                    if handle_input(&fdelta, &input, &mut state, &renderer) {
                        *control_flow = ControlFlow::Exit;
                    }
                    input.end();

                    state.frame_timer.start();
                    renderer.render(&state, resized);
                    state.frame_timer.stop();

                    resized = false;
                }
            }
            _ => {}
        }
    });
}

