use crate::util::point::Point;

#[derive(Debug)]
pub struct CameraView {
    pub pos: Point<f32>,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug)]
pub enum ClientMessage {
    CameraUpdate(CameraView),
    AddEnergy(Point<usize>),
    Swap(Point<usize>, Point<usize>),
    Pause(bool),
    Step(),
    RenderFinished(),
}

pub enum WorldMessage {}
