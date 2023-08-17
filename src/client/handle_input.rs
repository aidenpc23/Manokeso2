use std::time::Duration;

use winit::event::MouseButton;

use super::{
    state::ClientState,
    input::Input,
    keybinds::{Action, Keybinds},
};

use crate::{
    message::ClientMessage,
    rsc::PLAYER_SPEED,
};

pub fn handle_input(
    delta: &Duration,
    input: &Input,
    client: &mut ClientState,
) -> bool {
    let ainput = (input, &client.keybinds);
    if ainput.pressed(Action::Exit) {
        return true;
    }

    // camera stuff

    if input.scroll_delta != 0.0 {
        client.camera_scroll += input.scroll_delta;
        client.camera.scale = (client.camera_scroll * 0.1).exp();
    }

    let delta_mult = delta.as_millis() as f32;
    let move_dist = PLAYER_SPEED * delta_mult / client.camera.scale;

    let pos = &mut client.camera.pos;
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

    // interactions

    if input.mouse_just_pressed(MouseButton::Left) {
        client.held_tile = client.hovered_tile;
    }

    if input.mouse_just_released(MouseButton::Left) {
        if let Some(tile1) = client.held_tile {
            if let Some(tile2) = client.hovered_tile {
                client.world.send(ClientMessage::Swap(tile1.pos, tile2.pos));
            }
        }
        client.held_tile = None;
    }

    if ainput.just_pressed(Action::AddEnergy) {
        if let Some(tile) = client.hovered_tile {
            client.world.send(ClientMessage::AddEnergy(tile.pos));
        }
    }

    if ainput.just_pressed(Action::Pause) {
        client.paused = !client.paused;
        client.world.send(ClientMessage::Pause(client.paused));
    }

    if ainput.just_pressed(Action::Step) {
        client.world.send(ClientMessage::Step());
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
