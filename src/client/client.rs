use std::{sync::mpsc::channel, time::Instant};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::{
    message::{ClientMessage, WorldMessage},
    world::World,
};

use super::{
    config::Config, handle_input::handle_input, input::Input, state::Client, update::update,
    TileUpdateData,
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
    let mut state = Client::new(Config::load(), &event_loop, cs, wr).await;

    world_pool.spawn(move || {
        World::new(ws, cr).run();
    });

    let mut last_frame = Instant::now();
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

                    receive_messages(&mut state);
                    handle_input(&fdelta, &input, &mut state);
                    input.end();
                    update(&mut state, &input, now);
                    render(&mut state, resized);

                    resized = false;
                    if state.exit {
                        *control_flow = ControlFlow::Exit;
                    }

                    state.timer.stop();
                }
            }
            _ => {}
        }
    });
}

pub fn receive_messages(state: &mut Client) {
    for msg in state.world.receiver.try_iter() {
        match msg {
            WorldMessage::ViewSwap(mut view) => {
                std::mem::swap(&mut view, &mut state.world.view);
                state.world.send(ClientMessage::ViewSwap(view));
                state.tiles_dirty = true;
            }
        }
    }
}

pub fn render(client: &mut Client, resized: bool) {
    client.renderer.start_encoder();
    let view = &mut client.world.view;
    client.state.camera.pos = client.state.player.pos;
    if let Some(cam_view) = client.renderer.update_world(
        if client.tiles_dirty {
            Some(TileUpdateData {
                slice: &view.slice,
                connex_numbers: &view.connex_numbers,
                stability: &view.stability,
                reactivity: &view.reactivity,
                energy: &view.energy,
            })
        } else {
            None
        },
        &client.state.camera,
        resized,
    ) {
        client.world.send(ClientMessage::CameraUpdate(cam_view));
    }
    client.tiles_dirty = false;
    let ui = client.ui.compile(&client);
    client.renderer.update_ui(&ui, resized);
    client.renderer.draw();
}
