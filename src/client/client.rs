use std::{sync::mpsc::channel, time::Instant};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::{
    message::ClientMessage, sync::TileInfo, util::point::Point, world::World,
};

use super::{
    config::Config, handle_input::handle_input, input::Input, state::ClientState, TileUpdateData,
};

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
    let mut last_debug = Instant::now();
    let mut input = Input::new();
    let mut resized = false;

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
                state.renderer.draw();
            }
            Event::MainEventsCleared => {
                let now = Instant::now();
                let fdelta = now - last_frame;
                if fdelta > state.frame_time {
                    last_frame = now;

                    state.timer.start();

                    let ddelta = now - last_debug;
                    if ddelta > state.debug_stats.period {
                        last_debug = now;
                        state.debug_stats.client_update_time =
                            state.timer.avg().as_secs_f32() * 1000.0;
                        state.debug_stats.world_update_time =
                            state.world.view_info.time_taken.as_secs_f32() * 1000.0;
                    }

                    if handle_input(&fdelta, &input, &mut state) {
                        *control_flow = ControlFlow::Exit;
                    }
                    input.end();

                    for msg in state.world.receiver.try_iter() {
                        match msg {}
                    }

                    state.renderer.start_encoder();
                    sync_board(&mut state, &input);
                    let ui = state.ui.compile(&state);
                    if let Some(cam_view) = state.renderer.update(&state.camera, &ui, resized) {
                        state.world.send(ClientMessage::CameraUpdate(cam_view));
                    }
                    state.renderer.draw();

                    resized = false;

                    state.timer.stop();
                }
            }
            _ => {}
        }
    });
}

pub fn sync_board(state: &mut ClientState, input: &Input) {
    if let Ok(view) = state.world.view_lock.try_lock() {
        let mut info = view.info;
        state.renderer.sync(
            &mut info.render_info,
            &TileUpdateData {
                connex_numbers: &view.connex_numbers,
                stability: &view.stability,
                reactivity: &view.reactivity,
                energy: &view.energy,
            },
        );
        info.render_info.dirty = false;

        state.world.send(ClientMessage::RenderFinished());
        state.world.view_info = view.info.clone();

        let mouse_world_pos = state.renderer.pixel_to_world(input.mouse_pixel_pos);
        let rinfo = view.info.render_info;
        let Point { x, y } = mouse_world_pos - rinfo.pos;
        state.hovered_tile = if x >= 0.0
            && y >= 0.0
            && x < rinfo.slice.width as f32
            && y < rinfo.slice.height as f32
        {
            let pos = Point::new(x as usize, y as usize);
            let i = pos.index(rinfo.slice.width);
            let pos = pos + rinfo.slice.start;
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
        } else {
            None
        };
    }
}
