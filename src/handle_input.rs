use std::time::Duration;

use crate::{
    input::Input,
    keybinds::{Action, Keybinds},
    rsc::PLAYER_SPEED,
    state::GameState, render::Renderer,
};

pub fn handle_input(delta: &Duration, input: &Input, state: &mut GameState, renderer: &Renderer) -> bool {
    let ainput = (input, &state.keybinds);
    if ainput.pressed(Action::Exit) {
        return true;
    }

    let camera = &mut state.camera;
    let delta_mult = delta.as_millis() as f32;
    let move_dist = PLAYER_SPEED * delta_mult / camera.scale;


    let mouse_tile_pos = renderer.pixel_to_tile(input.mouse_pixel_pos);
    // state.hovered_tile = state.board.tile_at(mouse_tile_pos);
    // if input.mouse_just_pressed(MouseButton::Left) {
    //     state.held_tile = state.hovered_tile;
    // }
    // if input.mouse_just_released(MouseButton::Left) {
    //     if let Some(pos1) = state.held_tile {
    //         if let Some(pos2) = state.hovered_tile {
    //             state.board.player_swap(pos1, pos2);
    //         }
    //     }
    //     state.held_tile = None;
    // }

    if ainput.pressed(Action::MoveUp) {
        camera.pos.y += move_dist;
    }
    if ainput.pressed(Action::MoveLeft) {
        camera.pos.x -= move_dist;
    }
    if ainput.pressed(Action::MoveDown) {
        camera.pos.y -= move_dist;
    }
    if ainput.pressed(Action::MoveRight) {
        camera.pos.x += move_dist;
    }

    if ainput.just_pressed(Action::Pause) {
        state.paused = !state.paused;
    }
    // if ainput.just_pressed(Action::AddEnergy) {
    //     if let Some(pos) = state.hovered_tile {
    //         let i = pos.index(state.board.width());
    //         state
    //             .board
    //             .energy
    //             .god_set(i, state.board.energy.god_get(i) + 10.0);
    //     }
    // }
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
