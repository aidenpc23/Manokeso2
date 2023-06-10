use std::time::Duration;

use rand::random;
use winit::event::VirtualKeyCode as Key;

use crate::{input::Input, state::GameState, rsc::PLAYER_SPEED};

pub fn update(delta: &Duration, input: &Input, state: &mut GameState) -> bool {
    if input.pressed(Key::Escape) {
        return true;
    }
    let camera = &mut state.camera;
    let delta_mult = delta.as_millis() as f32;
    if input.pressed(Key::W) {
        camera.pos[1] += PLAYER_SPEED * delta_mult;
    }
    if input.pressed(Key::A) {
        camera.pos[0] -= PLAYER_SPEED * delta_mult;
    }
    if input.pressed(Key::R) {
        camera.pos[1] -= PLAYER_SPEED * delta_mult;
    }
    if input.pressed(Key::S) {
        camera.pos[0] += PLAYER_SPEED * delta_mult;
    }
    if input.just_pressed(Key::T) {
        state.colors.push((0..100).map(|i| random()).collect());
    }
    if input.just_pressed(Key::D) {
        state.colors.remove(0);
    }
    if input.just_pressed(Key::P) {
        state.colors[0][0] = [0.0; 3];
    }
    if input.scroll_delta != 0.0 {
        state.camera_scroll += input.scroll_delta;
        camera.scale = (state.camera_scroll * 0.1).exp();
    }
    return false;
}
