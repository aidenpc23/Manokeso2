use std::time::Duration;

use winit::event::VirtualKeyCode as Key;

use crate::{input::Input, state::GameState, rsc::PLAYER_SPEED};

pub fn handle_input(delta: &Duration, input: &Input, state: &mut GameState) -> bool {
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
    if input.just_pressed(Key::M) {
        println!("Pixel: {:?}", input.mouse_pixel_pos);
        println!("World: {:?}", input.mouse_world_pos);
    }
    if input.just_pressed(Key::T) {
        println!("total: {:?}", state.timers.total.avg());
        println!("1. update: {:?}", state.timers.update.avg());
        println!("2. render:");
        println!("   1. extract: {:?}", state.timers.render_extract.avg());
        println!("   2. write: {:?}", state.timers.render_write.avg());
        println!("   3. draw: {:?}", state.timers.render_draw.avg());
    }
    if input.just_pressed(Key::I) {
        if let Some(pos) = state.board.tile_at(input.mouse_world_pos) {
            let b = &state.board;
            let i = pos[0] + pos[1] * b.width();
            println!("tile pos: {:?}", pos);
            println!("connex number: {:?}", b.connex_numbers.read()[i]);
            println!("conductivity: {:?}", b.conductivity.read()[i]);
            println!("reactivity: {:?}", b.reactivity.read()[i]);
            println!("energy: {:?}", b.energy.read()[i]);
        }
    }
    if input.just_pressed(Key::E) {
        println!("Total energy: {}", state.board.total_energy());
    }
    if input.scroll_delta != 0.0 {
        state.camera_scroll += input.scroll_delta;
        camera.scale = (state.camera_scroll * 0.1).exp();
    }
    return false;
}
