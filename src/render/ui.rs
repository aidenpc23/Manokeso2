use super::primitive::{RoundedRectInstance, TextElement};

pub struct RenderableUI {
    pub text: Vec<TextElement>,
    pub rounded_rects: Vec<RoundedRectInstance>,
}
