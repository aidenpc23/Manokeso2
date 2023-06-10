use std::collections::HashSet;

use winit::event::{ElementState, MouseScrollDelta, VirtualKeyCode, WindowEvent};

pub struct Input {
    keys_down: HashSet<VirtualKeyCode>,
    pub scroll_delta: f32,
}

impl Input {
    pub fn new() -> Self {
        Self {
            keys_down: HashSet::new(),
            scroll_delta: 0.,
        }
    }
    pub fn update(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(code) = input.virtual_keycode {
                    match input.state {
                        ElementState::Pressed => self.keys_down.insert(code),
                        ElementState::Released => self.keys_down.remove(&code),
                    };
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                self.scroll_delta = match delta {
                    MouseScrollDelta::LineDelta(_, v) => v,
                    MouseScrollDelta::PixelDelta(v) => v.y as f32,
                };
            }
            _ => (),
        }
    }
    pub fn end(&mut self) {
        self.scroll_delta = 0.0;
    }
    pub fn down(&self, key: VirtualKeyCode) -> bool {
        self.keys_down.contains(&key)
    }
}
