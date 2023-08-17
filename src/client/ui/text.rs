use crate::{client::ClientState, render::TextElement, util::point::Point};

pub type TextUpdater = fn(&ClientState) -> String;

pub struct Text {
    pub content: TextUpdater,
    pub align: Align,
    pub pos: fn((f32, f32)) -> Point<f32>,
    pub bounds: fn((f32, f32)) -> (f32, f32),
}

impl Text {
    pub fn into_element(&self, state: &ClientState) -> TextElement {
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

#[derive(Clone, Copy)]
pub enum Align {
    Left,
    Center,
    Right,
}

