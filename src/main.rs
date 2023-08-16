use std::{time, sync::mpsc::channel};

use client::Client;
use config::Config;
use handle_input::handle_input;
use input::Input;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};

mod board_view;
mod camera;
mod client;
mod config;
mod handle_input;
mod input;
mod keybinds;
mod message;
mod render;
mod rsc;
mod util;
mod world;

use render::Renderer;
use world::World;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    // Setup
    env_logger::init();
    let (cs, cr) = channel();
    let mut client = Client::new(Config::load(), cs);

    let event_loop = EventLoop::new();
    let mut renderer = Renderer::new(&event_loop, &client.camera).await;
    renderer.window.set_visible(true);

    let mut last_frame = time::Instant::now();
    let mut input = Input::new();
    let mut resized = false;

    let cv = client.client_view.clone();
    let bv = client.board_view.clone();

    std::thread::spawn(move || {
        let mut server = World::new(cv, bv, cr);
        server.run();
    });

    // Game loop
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
            Event::RedrawRequested(_) => renderer.render(&client, false),
            Event::MainEventsCleared => {
                let now = time::Instant::now();
                let fdelta = now - last_frame;
                if fdelta > client.frame_time {
                    last_frame = now;

                    if handle_input(&fdelta, &input, &mut client, &renderer) {
                        *control_flow = ControlFlow::Exit;
                    }
                    input.end();

                    if let Ok(mut cview) = client.client_view.try_write() {
                        cview.paused = client.paused;
                    }

                    client.frame_timer.start();
                    renderer.render(&client, resized);
                    client.frame_timer.stop();

                    resized = false;
                }
            }
            _ => {}
        }
    });
}
