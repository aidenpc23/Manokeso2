use std::time::Duration;

use winit::event::MouseButton;

use crate::{
    client::Client,
    input::Input,
    keybinds::{Action, Keybinds},
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

    let camera = &mut client.camera;
    let delta_mult = delta.as_millis() as f32;
    let move_dist = PLAYER_SPEED * delta_mult / camera.scale;

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
                if let Err(..) = client
                    .sender
                    .send(crate::message::ClientMessage::Swap(pos1, pos2))
                {
                    println!("Failed to send swap to server!");
                }
            }
        }
        client.held_tile = None;
    }

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
        client.paused = !client.paused;
    }
    if ainput.just_pressed(Action::AddEnergy) {
        if let Some(pos) = client.hovered_tile {
            if let Err(..) = client
                .sender
                .send(crate::message::ClientMessage::AddEnergy(pos)) {
                    println!("Failed to send add energy to server!");
                }
        }
    }
    if ainput.just_pressed(Action::Step) {
        client.step = true;
    }

    if input.scroll_delta != 0.0 {
        client.camera_scroll += input.scroll_delta;
        camera.scale = (client.camera_scroll * 0.1).exp();
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
