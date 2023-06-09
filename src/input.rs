use std::collections::HashSet;

use winit::event::{ElementState, VirtualKeyCode, WindowEvent};

pub struct Input {
    keys_down: HashSet<VirtualKeyCode>,
}

impl Input {
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
            _ => (),
        }
    }
    pub fn down(&self, key: VirtualKeyCode) -> bool {
        self.keys_down.contains(&key)
    }
}
