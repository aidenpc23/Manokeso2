use crate::{util::point::Point, client::ui::element::Align};

pub struct UIPrimatives {
    pub text: Vec<TextPrimitive>,
    pub rounded_rects: Vec<RoundedRectPrimitive>,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RoundedRectPrimitive {
    pub top_left: UIPoint,
    pub bottom_right: UIPoint,
    pub colors: [[f32; 4]; 4],
    pub radius: f32,
    pub inner_radius: f32,
    pub thickness: f32,
}

#[derive(PartialEq, Clone)]
pub struct TextPrimitive {
    pub content: String,
    pub align: Align,
    pub pos: Point<f32>,
    pub bounds: (f32, f32),
}

impl TextPrimitive {
    pub fn empty() -> Self {
        Self {
            content: String::new(),
            align: Align::Left,
            pos: Point::default(),
            bounds: (0.0, 0.0),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UIPoint {
    pub anchor: Point<f32>,
    pub offset: Point<f32>,
}

impl UIPoint {
    pub fn anchor_offset(anchor_x: f32, anchor_y: f32, offset_x: f32, offset_y: f32) -> Self {
        Self {
            anchor: Point::new(anchor_x, anchor_y),
            offset: Point::new(offset_x, offset_y),
        }
    }
}

