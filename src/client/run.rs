use std::time::Instant;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::{
    board::BoardWorker,
    common::{
        interface::interface_pair,
        message::{WorkerCommand, WorkerResponse},
    },
};

use super::{client::Client, config::Config, input::Input, TileUpdateData};

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
            BoardWorker::new(ci).run();
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

                        client.receive_messages();
                        client.handle_input(&fdelta, &input);
                        input.end();
                        client.update(&input, now);
                        client.render(resized);

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

    fn receive_messages(&mut self) {
        for msg in self.worker.receiver.try_iter() {
            match msg {
                WorkerResponse::ViewSwap(mut view) => {
                    std::mem::swap(&mut view, &mut self.worker.view);
                    self.worker.send(WorkerCommand::ViewSwap(view));
                    self.view_dirty = true;
                }
                WorkerResponse::Loaded(state) => {
                    self.state = state;
                    self.paused = true;
                }
            }
        }
    }

    fn render(&mut self, resized: bool) {
        self.renderer.start_encoder();
        let view = &mut self.worker.view;
        self.state.camera.pos = self.state.player.pos;
        if let Some(cam_view) = self.renderer.update(
            if self.view_dirty {
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
            &self.state.camera,
            resized,
        ) {
            self.worker.send(WorkerCommand::CameraUpdate(cam_view));
        }
        self.view_dirty = false;
        let ui = self.ui.compile(&self);
        self.renderer.update_ui(&ui, resized);
        self.renderer.draw();
    }
}
