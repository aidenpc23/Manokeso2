use std::time::Instant;

use winit::{event::WindowEvent, event_loop::ActiveEventLoop};

use crate::common::message::{WorkerCommand, WorkerResponse};

use super::{client::Client, TileUpdateData};

impl Client<'_> {
    pub fn update(&mut self, event_loop: &ActiveEventLoop) {
        let now = Instant::now();
        if now > self.target {
            self.target += self.frame_time;

            self.timer.start();

            let time_delta = now - self.last_update;
            self.last_update = now;

            self.receive_messages();
            self.handle_input(&time_delta);
            self.input.end();
            self.update_world(now);
            self.render(self.resized);

            if self.exit {
                self.worker.send(WorkerCommand::Exit());
                for _ in self.worker.receiver.iter() {}
                event_loop.exit();
            }

            self.timer.stop();
        }
    }

    pub fn window_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => self.exit = true,
            WindowEvent::Resized(size) => self.resized = true,
            WindowEvent::RedrawRequested => self.renderer.draw(),
            WindowEvent::CursorLeft { .. } => {
                self.input.clear();
            }
            _ => self.input.update_window(event),
        }
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
        let view = &mut self.worker.view;
        self.state.camera.pos = self.state.player.pos;
        if let Some(cam_view) = self.renderer.update(
            if self.view_dirty {
                Some(TileUpdateData {
                    slice: &view.slice,
                    connex_numbers: &view.bufs.connex_numbers,
                    stability: &view.bufs.stability,
                    reactivity: &view.bufs.reactivity,
                    energy: &view.bufs.energy,
                    omega: &view.bufs.omega,
                    gamma: &view.bufs.gamma,
                    delta: &view.bufs.delta,
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
        let ui = self.ui.compile(self);
        self.renderer.update_ui(&ui, resized);
        self.renderer.draw();
    }
}
