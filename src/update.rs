use std::time::Duration;

use winit::event::VirtualKeyCode as Key;

use crate::{input::Input, state::GameState, rsc::PLAYER_SPEED};

pub fn update(delta: &Duration, input: &Input, state: &mut GameState) -> bool {
    if input.down(Key::Escape) {
        return true;
    }
    let camera = &mut state.camera;
    let delta_mult = delta.as_millis() as f32;
    if input.down(Key::W) {
        camera.pos[1] += PLAYER_SPEED * delta_mult;
    }
    if input.down(Key::A) {
        camera.pos[0] -= PLAYER_SPEED * delta_mult;
    }
    if input.down(Key::R) {
        camera.pos[1] -= PLAYER_SPEED * delta_mult;
    }
    if input.down(Key::S) {
        camera.pos[0] += PLAYER_SPEED * delta_mult;
    }
    if input.scroll_delta != 0.0 {
        state.camera_scroll += input.scroll_delta;
        camera.scale = (state.camera_scroll * 0.1).exp();
    }
    return false;
}
