use std::time::Duration;

use winit::event::MouseButton;

use crate::{
    client::Client,
    input::Input,
    keybinds::{Action, Keybinds},
    message::ClientMessage,
    render::Renderer,
    rsc::PLAYER_SPEED,
    util::point::Point,
};

pub fn handle_input(
    delta: &Duration,
    input: &Input,
    client: &mut Client,
    renderer: &Renderer,
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

    let mouse_world_pos = renderer.pixel_to_world(input.mouse_pixel_pos);
    if let Ok(view) = client.board_view.try_read() {
        let Point { x, y } = mouse_world_pos - view.pos;
        client.hovered_tile =
            if x < 0.0 || y < 0.0 || x >= view.slice.width as f32 || y >= view.slice.height as f32 {
                None
            } else {
                Some(Point::new(x as usize, y as usize))
            }
    }

    if input.mouse_just_pressed(MouseButton::Left) {
        client.held_tile = client.hovered_tile;
    }

    if input.mouse_just_released(MouseButton::Left) {
        if let Some(pos1) = client.held_tile {
            if let Some(pos2) = client.hovered_tile {
                client.send(ClientMessage::Swap(pos1, pos2));
            }
        }
        client.held_tile = None;
    }

    if ainput.just_pressed(Action::AddEnergy) {
        if let Some(pos) = client.hovered_tile {
            client.send(ClientMessage::AddEnergy(pos));
        }
    }

    if ainput.just_pressed(Action::Pause) {
        client.paused = !client.paused;
        client.send(ClientMessage::Pause(client.paused));
    }

    if ainput.just_pressed(Action::Step) {
        client.send(ClientMessage::Step());
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
