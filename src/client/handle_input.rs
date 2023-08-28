use std::time::Duration;

use winit::event::{MouseButton, VirtualKeyCode as Key};

use super::{
    input::Input,
    keybinds::{Action, Keybinds},
    state::ClientState,
};

use crate::message::{ClientMessage, TileChange::*};

pub fn handle_input(delta: &Duration, input: &Input, state: &mut ClientState) {
    let ainput = (input, &state.keybinds);
    if ainput.pressed(Action::Exit) {
        state.exit = true;
    }

    // camera stuff

    if input.scroll_delta != 0.0 {
        state.camera_scroll += input.scroll_delta;
        if !state.player.creative {
            state.camera_scroll = state.camera_scroll.clamp(0.0, 30.0);
        }
        state.camera.scale = (state.camera_scroll * 0.1).exp();
    }

    // interactions

    if !state.paused || state.player.creative {
        let pos = &mut state.player.pos;
        let delta_mult = delta.as_millis() as f32;
        let move_dist = state.player.speed * delta_mult / state.camera.scale;

        if ainput.pressed(Action::MoveUp) {
            pos.y += move_dist;
        }
        if ainput.pressed(Action::MoveLeft) {
            pos.x -= move_dist;
        }
        if ainput.pressed(Action::MoveDown) {
            pos.y -= move_dist;
        }
        if ainput.pressed(Action::MoveRight) {
            pos.x += move_dist;
        }

        if input.mouse_just_pressed(MouseButton::Left) {
            state.selected_tile = state.hovered_tile;
        }

        if input.mouse_just_pressed(MouseButton::Right) {
            if let Some(tile1) = state.selected_tile {
                if let Some(tile2) = state.hovered_tile {
                    state.world.send(ClientMessage::Swap(tile1.pos, tile2.pos, state.player.creative));
                }
            }
            state.selected_tile = None;
        }
    }

    if state.player.creative {
        if input.just_pressed(Key::T) {
            state.world.send(ClientMessage::Save());
        }
        if input.just_pressed(Key::G) {
            state.world.send(ClientMessage::Load());
        }

        if input.just_pressed(Key::Y) {
            if let Some(tile) = state.hovered_tile {
                state
                    .world
                    .send(ClientMessage::ChangeTile(tile.pos, ConnexNumber(1)));
            }
        }

        if input.just_pressed(Key::H) {
            if let Some(tile) = state.hovered_tile {
                state
                    .world
                    .send(ClientMessage::ChangeTile(tile.pos, ConnexNumber(-1)));
            }
        }

        if input.just_pressed(Key::U) {
            if let Some(tile) = state.hovered_tile {
                state
                    .world
                    .send(ClientMessage::ChangeTile(tile.pos, Stability(0.1)));
            }
        }

        if input.just_pressed(Key::J) {
            if let Some(tile) = state.hovered_tile {
                state
                    .world
                    .send(ClientMessage::ChangeTile(tile.pos, Stability(-0.1)));
            }
        }

        if input.just_pressed(Key::I) {
            if let Some(tile) = state.hovered_tile {
                state
                    .world
                    .send(ClientMessage::ChangeTile(tile.pos, Reactivity(0.1)));
            }
        }

        if input.just_pressed(Key::K) {
            if let Some(tile) = state.hovered_tile {
                state
                    .world
                    .send(ClientMessage::ChangeTile(tile.pos, Reactivity(-0.1)));
            }
        }

        if input.just_pressed(Key::O) {
            if let Some(tile) = state.hovered_tile {
                state
                    .world
                    .send(ClientMessage::ChangeTile(tile.pos, Energy(20.0)));
            }
        }

        if input.just_pressed(Key::L) {
            if let Some(tile) = state.hovered_tile {
                state
                    .world
                    .send(ClientMessage::ChangeTile(tile.pos, Energy(-20.0)));
            }
        }

        if input.just_pressed(Key::P) {
            if let Some(tile) = state.hovered_tile {
                state
                    .world
                    .send(ClientMessage::ChangeTile(tile.pos, Delta(1)));
            }
        }

        if input.just_pressed(Key::Semicolon) {
            if let Some(tile) = state.hovered_tile {
                state
                    .world
                    .send(ClientMessage::ChangeTile(tile.pos, Delta(-1)));
            }
        }

        if ainput.just_pressed(Action::Step) {
            state.world.send(ClientMessage::Step());
        }
    }

    if state.player.admin {
        if input.just_pressed(Key::B) {
            state.player.creative = !state.player.creative;
            if !state.player.creative {
                state.camera_scroll = state.camera_scroll.clamp(0.0, 30.0);
                state.camera.scale = (state.camera_scroll * 0.1).exp();
            }
        }
    }

    if ainput.just_pressed(Action::Pause) {
        state.paused = !state.paused;
        state.world.send(ClientMessage::Pause(state.paused));
    }
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
