use std::time::Duration;

pub const WIDTH: usize = 1000;
pub const HEIGHT: usize = 1000;
pub const PLAYER_SPEED: f32 = 0.01;
pub const FPS: u32 = 60;
pub const FRAME_TIME: Duration = Duration::from_millis(1000/FPS as u64);

