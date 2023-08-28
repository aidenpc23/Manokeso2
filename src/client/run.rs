use std::time::Instant;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::{
    board::BoardWorker,
    common::{message::{WorkerCommand, WorkerResponse}, interface::interface_pair},
};

use super::{
    client::Client, config::Config, handle_input::handle_input, input::Input, update::update,
    TileUpdateData,
};

impl Client {
    pub async fn run() {
        let worker_thread_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(rayon::current_num_threads() - 1)
            .build()
            .unwrap();
        // Setup

        let event_loop = EventLoop::new();
        let (wi, ci) = interface_pair();
        let mut client = Client::new(Config::load(), &event_loop, wi).await;

        worker_thread_pool.spawn(move || {
            BoardWorker::new(ci)
            .run();
        });

        let mut last_frame = Instant::now();
        let mut input = Input::new();
        let mut resized = false;

        client.renderer.window.set_visible(true);

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { event, window_id }
                    if window_id == client.renderer.window.id() =>
                {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(_) => resized = true,
                        _ => input.update(event),
                    }
                }
                Event::RedrawRequested(_) => {
                    client.renderer.start_encoder();
                    client.renderer.draw();
                }
                Event::MainEventsCleared => {
                    let now = Instant::now();
                    let fdelta = now - last_frame;
                    if fdelta > client.frame_time {
                        last_frame = now;

                        client.timer.start();

                        Self::receive_messages(&mut client);
                        handle_input(&fdelta, &input, &mut client);
                        input.end();
                        update(&mut client, &input, now);
                        Self::render(&mut client, resized);

                        resized = false;
                        if client.exit {
                            *control_flow = ControlFlow::Exit;
                        }

                        client.timer.stop();
                    }
                }
                _ => {}
            }
        });
    }

    pub fn receive_messages(client: &mut Client) {
        for msg in client.worker.receiver.try_iter() {
            match msg {
                WorkerResponse::ViewSwap(mut view) => {
                    std::mem::swap(&mut view, &mut client.worker.view);
                    client.worker.send(WorkerCommand::ViewSwap(view));
                    client.view_dirty = true;
                }
                WorkerResponse::Loaded(state) => {
                    client.state = state;
                    client.paused = true;
                },
            }
        }
    }

    pub fn render(client: &mut Client, resized: bool) {
        client.renderer.start_encoder();
        let view = &mut client.worker.view;
        client.state.camera.pos = client.state.player.pos;
        if let Some(cam_view) = client.renderer.update(
            if client.view_dirty {
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
            client.worker.send(WorkerCommand::CameraUpdate(cam_view));
        }
        client.view_dirty = false;
        let ui = client.ui.compile(&client);
        client.renderer.update_ui(&ui, resized);
        client.renderer.draw();
    }
}
