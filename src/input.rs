use std::collections::HashSet;

use winit::event::{ElementState, MouseScrollDelta, VirtualKeyCode, WindowEvent};

pub struct Input {
    pub mouse_pos: [f32; 2],
    just_pressed: HashSet<VirtualKeyCode>,
    pressed: HashSet<VirtualKeyCode>,
    pub scroll_delta: f32,
}

impl Input {
    pub fn new() -> Self {
        Self {
            mouse_pos: [0.0, 0.0],
            just_pressed: HashSet::new(),
            pressed: HashSet::new(),
            scroll_delta: 0.,
        }
    }
    pub fn update(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(code) = input.virtual_keycode {
                    match input.state {
                        ElementState::Pressed => {
                            self.just_pressed.insert(code);
                            self.pressed.insert(code);
                        }
                        ElementState::Released => {
                            self.pressed.remove(&code);
                        }
                    };
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                self.scroll_delta = match delta {
                    MouseScrollDelta::LineDelta(_, v) => v,
                    MouseScrollDelta::PixelDelta(v) => (v.y / 2.0) as f32,
                };
            }
            WindowEvent::CursorLeft { .. } => {
                self.pressed.clear();
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos = [position.x as f32, position.y as f32];
            }
            _ => (),
        }
    }
    pub fn end(&mut self) {
        self.scroll_delta = 0.0;
        self.just_pressed.clear();
    }
    pub fn pressed(&self, key: VirtualKeyCode) -> bool {
        self.pressed.contains(&key)
    }
    pub fn just_pressed(&self, key: VirtualKeyCode) -> bool {
        self.just_pressed.contains(&key)
    }
}
