use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};

mod res;
mod window;

use window::GameWindow;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    // Setup logger, event loop, and window
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut state: GameWindow = GameWindow::new(&event_loop).await;
    state.window.set_visible(true);

    // Game loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == state.window.id() => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                window_id,
            } if window_id == state.window.id() => {
                if input.virtual_keycode == Some(VirtualKeyCode::Escape) {
                    *control_flow = ControlFlow::Exit
                }
            }
            Event::RedrawRequested(_) => state.render(),
            _ => (),
        }
    });
}
