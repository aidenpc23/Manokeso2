use std::collections::HashSet;

use winit::event::{ElementState, MouseScrollDelta, VirtualKeyCode, WindowEvent};

use crate::render::Renderer;

pub struct Input {
    pub mouse_world_pos: [f32; 2],
    pub mouse_pixel_pos: [f32; 2],
    just_pressed: HashSet<VirtualKeyCode>,
    pressed: HashSet<VirtualKeyCode>,
    pub scroll_delta: f32,
}

impl Input {
    pub fn new() -> Self {
        Self {
            mouse_pixel_pos: [0.0, 0.0],
            mouse_world_pos: [0.0, 0.0],
            just_pressed: HashSet::new(),
            pressed: HashSet::new(),
            scroll_delta: 0.,
        }
    }
    pub fn update(&mut self, event: WindowEvent, renderer: &Renderer) {
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
                self.mouse_pixel_pos = [position.x as f32, position.y as f32];
                self.mouse_world_pos = renderer.pixel_to_world(self.mouse_pixel_pos);
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
