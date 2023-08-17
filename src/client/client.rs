use std::{sync::mpsc::channel, time::Instant};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::{world::World, util::point::Point, sync::TileInfo};

use super::{state::ClientState, config::Config, input::Input, handle_input::handle_input, ui::layout};

pub async fn run() {
    let world_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(rayon::current_num_threads() - 1)
        .build()
        .unwrap();
    // Setup
    let (cs, cr) = channel();
    let (ws, wr) = channel();

    let event_loop = EventLoop::new();
    let mut state = ClientState::new(Config::load(), &event_loop, cs, wr).await;

    let bv = state.world.view_lock.clone();

    world_pool.spawn(move || {
        World::new(bv, ws, cr).run();
    });

    let mut last_frame = Instant::now();
    let mut input = Input::new();
    let mut resized = false;
    let text = layout::BOARD;
    let mut text_elements = Vec::new();

    state.renderer.window.set_visible(true);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, window_id } if window_id == state.renderer.window.id() => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(_) => resized = true,
                    _ => input.update(event),
                }
            }
            Event::RedrawRequested(_) => {
                state.renderer.start_encoder();
                state.renderer.render(&state.world, &state.camera, &text_elements, false);
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

                    state.timer.start();

                    for msg in state.world.receiver.try_iter() {
                        match msg {}
                    }

                    state.renderer.start_encoder();
                    let mouse_world_pos = state.renderer.pixel_to_world(input.mouse_pixel_pos);
                    if let Ok(mut view) = state.world.view_lock.try_lock() {
                        state.renderer.sync(&mut view);
                        state.world.send(crate::message::ClientMessage::RenderFinished());
                        state.world.view_info = view.info.clone();
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
                            Some(TileInfo {
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
                    text_elements = text.iter().map(|t| t.into_element(&state)).collect();
                    state.renderer.render(&state.world, &state.camera, &text_elements, resized);
                    state.timer.stop();

                    resized = false;
                }
            }
            _ => {}
        }
    });
}
