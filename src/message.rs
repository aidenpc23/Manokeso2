use crate::util::point::Point;

pub struct CameraView {
    pub pos: Point<f32>,
    pub width: f32,
    pub height: f32,
}

pub enum ClientMessage {
    CameraUpdate(CameraView),
    AddEnergy(Point<usize>),
    Swap(Point<usize>, Point<usize>),
    Pause(bool),
    Step(),
}
