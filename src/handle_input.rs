use std::time::Duration;

use winit::event::MouseButton;

use crate::{
    input::Input,
    keybinds::{Action, Keybinds},
    rsc::PLAYER_SPEED,
    state::GameState,
};

pub fn handle_input(delta: &Duration, input: &Input, state: &mut GameState) -> bool {
    let ainput = (input, &state.keybinds);
    if ainput.pressed(Action::Exit) {
        return true;
    }

    let camera = &mut state.camera;
    let delta_mult = delta.as_millis() as f32;
    let move_dist = PLAYER_SPEED * delta_mult / camera.scale;

    if input.mouse_just_pressed(MouseButton::Left) {
        state.selected_tile = state.board.tile_at(input.mouse_tile_pos);
    }
    if input.mouse_just_released(MouseButton::Left) {
        if let Some(pos1) = state.selected_tile {
            if let Some(pos2) = state.board.tile_at(input.mouse_tile_pos) {
                state.board.player_swap(pos1, pos2);
            }
        }
        state.selected_tile = None;
    }

    if ainput.pressed(Action::MoveUp) {
        camera.pos[1] += move_dist;
    }
    if ainput.pressed(Action::MoveLeft) {
        camera.pos[0] -= move_dist;
    }
    if ainput.pressed(Action::MoveDown) {
        camera.pos[1] -= move_dist;
    }
    if ainput.pressed(Action::MoveRight) {
        camera.pos[0] += move_dist;
    }

    if ainput.just_pressed(Action::Pause) {
        state.paused = !state.paused;
    }
    if ainput.just_pressed(Action::AddEnergy) {
        if let Some(pos) = state.board.tile_at(input.mouse_tile_pos) {
            let i = pos[1] * state.board.width() + pos[0];
            state.board.energy.god_set(i, state.board.energy.god_get(i) + 10.0);
        }
    }
    if ainput.just_pressed(Action::Step) {
        state.step = true;
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
