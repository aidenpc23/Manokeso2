use crate::{
    client::Client,
    render::primitive::{RoundedRectPrimitive, TextPrimitive, UIPoint},
    util::point::Point,
};

pub struct UIElement<T> {
    pub top_left: UIPoint,
    pub bottom_right: UIPoint,
    pub data: T
}

pub struct RoundedRect {
    pub top_left: UIPoint,
    pub bottom_right: UIPoint,
    pub colors: [[f32; 4]; 4],
    pub radius: f32,
    pub inner_radius: f32,
    pub thickness: f32,
}

impl RoundedRect {
    pub fn to_primitive(&self) -> RoundedRectPrimitive {
        RoundedRectPrimitive {
            top_left: self.top_left,
            bottom_right: self.bottom_right,
            colors: self.colors,
            radius: self.radius,
            inner_radius: self.inner_radius,
            thickness: self.thickness,
        }
    }
}

impl Default for RoundedRect {
    fn default() -> Self {
        Self {
            top_left: UIPoint {
                anchor: Point::zero(),
                offset: Point::zero(),
            },
            bottom_right: UIPoint {
                anchor: Point { x: 1.0, y: 1.0 },
                offset: Point::zero(),
            },
            colors: [
                [0.0, 0.0, 0.0, 1.0],
                [0.0, 0.0, 0.0, 1.0],
                [0.0, 0.0, 0.0, 1.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            radius: 0.0,
            thickness: 0.0,
            inner_radius: 0.0,
        }
    }
}


pub type TextUpdater = fn(&Client) -> String;

pub struct Text {
    pub content: TextUpdater,
    pub align: Align,
    pub pos: fn((f32, f32)) -> Point<f32>,
    pub bounds: fn((f32, f32)) -> (f32, f32),
}

impl Text {
    pub fn into_primitive(&self, client: &Client) -> TextPrimitive {
        let size = client.renderer.window.inner_size();
        let bounds = (size.width as f32, size.height as f32);
        let text_bounds = (self.bounds)(bounds);
        TextPrimitive {
            pos: (self.pos)(bounds),
            bounds: text_bounds,
            align: self.align,
            content: (self.content)(client),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Align {
    Left,
    Center,
    Right,
}
