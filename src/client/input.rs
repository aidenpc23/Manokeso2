use std::collections::HashSet;

use winit::event::{ElementState, MouseScrollDelta, VirtualKeyCode, WindowEvent, MouseButton};

use crate::util::point::Point;

pub struct Input {
    pub mouse_pixel_pos: Point<f32>,

    pressed: HashSet<VirtualKeyCode>,
    just_pressed: HashSet<VirtualKeyCode>,

    mouse_pressed: HashSet<MouseButton>,
    mouse_just_pressed: HashSet<MouseButton>,
    mouse_just_released: HashSet<MouseButton>,

    pub scroll_delta: f32,
}

impl Input {
    pub fn new() -> Self {
        Self {
            mouse_pixel_pos: Point::zero(),
            pressed: HashSet::new(),
            just_pressed: HashSet::new(),
            mouse_pressed: HashSet::new(),
            mouse_just_pressed: HashSet::new(),
            mouse_just_released: HashSet::new(),
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
                self.mouse_pixel_pos = Point::new(position.x as f32, position.y as f32);
            }
            WindowEvent::MouseInput { button, state, .. } => {
                match state {
                    ElementState::Pressed => {
                        self.mouse_just_pressed.insert(button);
                        self.mouse_pressed.insert(button);
                    }
                    ElementState::Released => {
                        self.mouse_pressed.remove(&button);
                        self.mouse_just_released.insert(button);
                    }
                }
            }
            _ => (),
        }
    }

    pub fn end(&mut self) {
        self.scroll_delta = 0.0;
        self.just_pressed.clear();
        self.mouse_just_pressed.clear();
        self.mouse_just_released.clear();
    }

    pub fn pressed(&self, key: VirtualKeyCode) -> bool {
        self.pressed.contains(&key)
    }

    pub fn just_pressed(&self, key: VirtualKeyCode) -> bool {
        self.just_pressed.contains(&key)
    }

    #[allow(dead_code)]
    pub fn mouse_pressed(&self, button: MouseButton) -> bool {
        self.mouse_pressed.contains(&button)
    }

    pub fn mouse_just_pressed(&self, button: MouseButton) -> bool {
        self.mouse_just_pressed.contains(&button)
    }

    #[allow(dead_code)]
    pub fn mouse_just_released(&self, button: MouseButton) -> bool {
        self.mouse_just_released.contains(&button)
    }
}
