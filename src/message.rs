use crate::util::point::Point;

pub enum ClientMessage {
    AddEnergy(Point<usize>),
    Swap(Point<usize>, Point<usize>)
}

