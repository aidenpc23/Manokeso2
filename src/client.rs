use std::{sync::mpsc::channel, time::Instant};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::{
    config::Config, handle_input::handle_input, input::Input, render::Renderer, state::ClientState,
    util::point::Point, world::World,
};

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
            Event::RedrawRequested(_) => {
                renderer.start_encoder();
                renderer.render(&state, false);
            }
            Event::MainEventsCleared => {
                let now = Instant::now();
                let fdelta = now - last_frame;
                if fdelta > state.frame_time {
                    last_frame = now;

                    if handle_input(&fdelta, &input, &mut state) {
                        *control_flow = ControlFlow::Exit;
                    }
                    input.end();

                    state.frame_timer.start();
                    renderer.start_encoder();
                    let mouse_world_pos = renderer.pixel_to_world(input.mouse_pixel_pos);
                    if let Ok(mut view) = state.board_view.try_write() {
                        renderer.sync(&mut view);
                        state.view_info = view.info.clone();
                        let Point { x, y } = mouse_world_pos - view.info.pos;
                        state.hovered_tile = if x < 0.0
                            || y < 0.0
                            || x >= view.info.slice.width as f32
                            || y >= view.info.slice.height as f32
                        {
                            None
                        } else {
                            let pos = Point::new(x as usize, y as usize);
                            let i = pos.index(view.info.slice.width);
                            let pos = pos + view.info.slice.start;
                            Some(crate::state::TileInfo {
                                pos,
                                connex_number: view.connex_numbers[i],
                                stability: view.stability[i],
                                reactivity: view.reactivity[i],
                                energy: view.energy[i],
                                alpha: view.alpha[i],
                                beta: view.beta[i],
                                gamma: view.gamma[i],
                                delta: view.delta[i],
                                omega: view.omega[i],
                            })
                        };
                    }
                    renderer.render(&state, resized);
                    state.frame_timer.stop();

                    resized = false;
                }
            }
            _ => {}
        }
    });
}
