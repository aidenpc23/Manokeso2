use crate::{util::point::Point, client::ui::element::Align};

pub struct UIPrimatives {
    pub text: Vec<TextElement>,
    pub rounded_rects: Vec<RoundedRectInstance>,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RoundedRectInstance {
    pub top_left_anchor: Point<f32>,
    pub top_left_offset: Point<f32>,
    pub bottom_right_anchor: Point<f32>,
    pub bottom_right_offset: Point<f32>,
    pub colors: [[f32; 4]; 4],
    pub radius: f32,
    pub inner_radius: f32,
    pub thickness: f32,
}

#[derive(PartialEq, Clone)]
pub struct TextElement {
    pub content: String,
    pub align: Align,
    pub pos: Point<f32>,
    pub bounds: (f32, f32),
}

impl TextElement {
    pub fn empty() -> Self {
        Self {
            content: String::new(),
            align: Align::Left,
            pos: Point::default(),
            bounds: (0.0, 0.0),
        }
    }
}
