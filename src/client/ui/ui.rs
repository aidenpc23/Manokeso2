use crate::{client::ClientState, render::ui::RenderableUI};

use super::element::{RoundedRect, Text};

pub struct GameUI {
    pub text: Vec<Text>,
    pub shapes: Vec<RoundedRect>,
}

impl GameUI {
    pub fn compile(&self, state: &ClientState) -> RenderableUI {
        RenderableUI {
            text: self.text.iter().map(|t| t.into_primitive(state)).collect(),
            rounded_rects: self.shapes.iter().map(|t| t.to_primitive()).collect(),
        }
    }
}
