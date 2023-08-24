use crate::{client::ClientState,  util::point::Point, render::primitive::TextElement};

pub type TextUpdater = fn(&ClientState) -> String;

pub struct Text {
    pub content: TextUpdater,
    pub align: Align,
    pub pos: fn((f32, f32)) -> Point<f32>,
    pub bounds: fn((f32, f32)) -> (f32, f32),
}

impl Text {
    pub fn into_primitive(&self, state: &ClientState) -> TextElement {
        let size = state.renderer.window.inner_size();
        let bounds = (size.width as f32, size.height as f32);
        let text_bounds = (self.bounds)(bounds);
        TextElement {
            pos: (self.pos)(bounds),
            bounds: text_bounds,
            align: self.align,
            content: (self.content)(state)
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Align {
    Left,
    Center,
    Right,
}

