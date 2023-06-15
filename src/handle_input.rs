use std::time::Duration;

use crate::{input::Input, rsc::PLAYER_SPEED, state::GameState, keybinds::{Action, Keybinds}};

pub fn handle_input(delta: &Duration, input: &Input, state: &mut GameState) -> bool {
    let ainput = (input, &state.keybinds);
    if ainput.pressed(Action::Exit) {
        return true;
    }
    let camera = &mut state.camera;
    let delta_mult = delta.as_millis() as f32;
    if ainput.pressed(Action::MoveUp) {
        camera.pos[1] += PLAYER_SPEED * delta_mult;
    }
    if ainput.pressed(Action::MoveLeft) {
        camera.pos[0] -= PLAYER_SPEED * delta_mult;
    }
    if ainput.pressed(Action::MoveDown) {
        camera.pos[1] -= PLAYER_SPEED * delta_mult;
    }
    if ainput.pressed(Action::MoveRight) {
        camera.pos[0] += PLAYER_SPEED * delta_mult;
    }
    if ainput.just_pressed(Action::Timers) {
        let t = &state.timers;
        if t.update.ready() {
            let render = t.render_extract.avg() + t.render_write.avg() + t.render_draw.avg();
            let total = t.update.avg() + render;
            println!("total: {:?}", total);
            println!("1. update: {:?}", t.update.avg());
            println!("2. render: {:?}", render);
            println!("   1. extract: {:?}", t.render_extract.avg());
            println!("   2. write: {:?}", t.render_write.avg());
            println!("   3. draw: {:?}", t.render_draw.avg());
        } else {
            println!("Not enough time has passed");
        }
    }
    if ainput.just_pressed(Action::TileInfo) {
        if let Some(pos) = state.board.tile_at(input.mouse_world_pos) {
            let b = &state.board;
            let i = pos[0] + pos[1] * b.width();
            println!("tile pos: {:?}", pos);
            println!("connex number: {:?}", b.connex_numbers.read()[i]);
            println!("stability: {:?}", b.stability.read()[i]);
            println!("reactivity: {:?}", b.reactivity.read()[i]);
            println!("energy: {:?}", b.energy.read()[i]);
        }
    }
    if ainput.just_pressed(Action::TotalEnergy) {
        println!("Total energy: {}", state.board.total_energy());
    }
    if input.scroll_delta != 0.0 {
        state.camera_scroll += input.scroll_delta;
        camera.scale = (state.camera_scroll * 0.1).exp();
    }
    return false;
}

trait ActionInput {
    fn pressed(&self, action: Action) -> bool;
    fn just_pressed(&self, action: Action) -> bool;
}

impl ActionInput for (&Input, &Keybinds) {
    fn pressed(&self, action: Action) -> bool {
        if let Some(key) = self.1.get(&action) {
            self.0.pressed(*key)
        } else {
            false
        }
    }
    fn just_pressed(&self, action: Action) -> bool {
        if let Some(key) = self.1.get(&action) {
            self.0.just_pressed(*key)
        } else {
            false
        }
    }
}
