use std::time::Duration;

pub const PLAYER_SPEED: f32 = 0.01;

pub const FPS: u32 = 60;
pub const UPS: u32 = 100;
pub const FRAME_TIME: Duration = Duration::from_millis(1000 / FPS as u64);
pub const UPDATE_TIME: Duration = Duration::from_millis(1000 / UPS as u64);

pub const CONNEX_NUMBER_RANGE: [u32; 2] = [0, 200];
pub const STABILITY_RANGE: [f32; 2] = [0.0, 1.0];
pub const REACTIVITY_RANGE: [f32; 2] = [-1.0, 1.0];
pub const ENERGY_RANGE: [f32; 2] = [0.0, 15.0];

pub const CLEAR_COLOR: wgpu::Color = wgpu::Color {
    r: 0.1,
    g: 0.1,
    b: 0.1,
    a: 1.0,
};

