use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};

mod camera;
mod res;
mod window;

use camera::Camera;
use window::GameWindow;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    // Setup logger, event loop, and window
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut camera = Camera::default();
    let mut state: GameWindow = GameWindow::new(&event_loop, &camera).await;
    state.window.set_visible(true);

    let mut scroll = 0.;

    // Game loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event,
                window_id,
            } if window_id == state.window.id() => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => {
                    if input.virtual_keycode == Some(VirtualKeyCode::Escape) {
                        *control_flow = ControlFlow::Exit
                    }
                    if input.virtual_keycode == Some(VirtualKeyCode::W) {
                        camera.pos[1] += 0.1;
                    }
                    if input.virtual_keycode == Some(VirtualKeyCode::A) {
                        camera.pos[0] -= 0.1;
                    }
                    if input.virtual_keycode == Some(VirtualKeyCode::R) {
                        camera.pos[1] -= 0.1;
                    }
                    if input.virtual_keycode == Some(VirtualKeyCode::S) {
                        camera.pos[0] += 0.1;
                    }
                    state.update_view(&camera);
                    state.render();
                },
                WindowEvent::Resized(s) => {
                    state.update_view(&camera);
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    scroll += match delta {
                        MouseScrollDelta::LineDelta(_, v) => v,
                        MouseScrollDelta::PixelDelta(v) => v.y as f32
                    };
                    camera.scale = (scroll * 0.1).exp();
                    state.update_view(&camera);
                    state.render();
                }
                _ => {}
            }
            Event::RedrawRequested(_) => state.render(),
            _ => (),
        }
    });
}
