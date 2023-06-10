use std::time::Duration;

pub const WIDTH: u32 = 1000;
pub const HEIGHT: u32 = 1000;
pub const FPS: u32 = 60;
pub const FRAME_TIME: Duration = Duration::from_millis(1000/FPS as u64);

