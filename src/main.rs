use state::State;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod render;
mod state;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    // Setup logger
    env_logger::init();
    let event_loop = EventLoop::new();
    // Create window and application state
    let window = WindowBuilder::new()
        .with_visible(false)
        .build(&event_loop)
        .unwrap();

    let mut state: State = State::new(window).await;

    // ==============================================
    // Define event loop functionality
    // ==============================================
    state.window.set_visible(true);

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

