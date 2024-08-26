use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::ControlFlow};

use crate::{board::BoardWorker, common::interface::interface_pair};

use super::{config::Config, Client};

pub struct ClientApp<'a> {
    client: Option<Client<'a>>,
}

impl<'a> ClientApp<'a> {
    fn client(&mut self) -> &mut Client<'a> {
        self.client.as_mut().expect("bruh")
    }

    pub fn new() -> Self {
        Self { client: None }
    }
}

impl ApplicationHandler for ClientApp<'_> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.client.is_none() {
            let worker_thread_pool = rayon::ThreadPoolBuilder::new()
                .num_threads((rayon::current_num_threads() - 1).max(1))
                .build()
                .unwrap();
            let (wi, ci) = interface_pair();
            let client = Client::new(Config::load(), event_loop, wi);
            self.client = Some(client);
            worker_thread_pool.spawn(move || {
                BoardWorker::new(ci).run();
            });
        }
        event_loop.set_control_flow(ControlFlow::Poll);
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        self.client().window_event(event);
    }

    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        self.client().input.update_device(event);
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.client().update(event_loop);
    }
}
