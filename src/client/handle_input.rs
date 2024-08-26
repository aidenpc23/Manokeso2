use std::time::Duration;

use winit::{event::MouseButton, keyboard::KeyCode as Key};

use super::{
    client::Client,
    input::Input,
    keybinds::{Action, Keybinds},
};

use crate::common::message::{TileChange::*, WorkerCommand};

impl Client<'_> {
    pub fn handle_input(&mut self, delta: &Duration) {
        let input = &self.input;

        let ainput = (input, &self.keybinds);
        if ainput.pressed(Action::Exit) {
            self.exit = true;
        }

        let state = &mut self.state;

        // camera stuff

        if input.scroll_delta != 0.0 {
            state.camera_scroll += input.scroll_delta;
            if !state.player.creative {
                state.camera_scroll = state.camera_scroll.clamp(2.0, 30.0);
            }
            state.camera.scale = (state.camera_scroll * 0.1).exp();
        }

        // interactions

        if !self.paused || state.player.creative {
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
                state.selected_tile = self.hovered_tile;
            }

            if input.mouse_just_pressed(MouseButton::Right)
                || input.mouse_just_released(MouseButton::Left)
            {
                if let (Some(selected), Some(hovered)) = (state.selected_tile, self.hovered_tile) {
                    if selected.pos != hovered.pos {
                        self.worker.send(WorkerCommand::Swap(
                            selected.pos,
                            hovered.pos,
                            state.player.creative,
                        ));
                        state.selected_tile = None;
                    }
                }
            }
        }

        if input.just_pressed(Key::KeyF) {
            self.renderer.window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
        }
        if !state.player.creative {
            if input.just_pressed(Key::KeyT) {
                let name = "save".to_string();
                self.worker.send(WorkerCommand::Save(name, state.clone()));
            }
            if input.just_pressed(Key::KeyG) {
                let name = "save".to_string();
                self.worker.send(WorkerCommand::Load(name));
            }
        }

        if state.player.creative {
            if input.just_pressed(Key::KeyT) {
                let name = "save".to_string();
                self.worker.send(WorkerCommand::Save(name, state.clone()));
            }
            if input.just_pressed(Key::KeyG) {
                let name = "save".to_string();
                self.worker.send(WorkerCommand::Load(name));
            }

            if input.just_pressed(Key::KeyY) {
                if let Some(tile) = self.hovered_tile {
                    self.worker
                        .send(WorkerCommand::ChangeTile(tile.pos, ConnexNumber(1)));
                }
            }

            if input.just_pressed(Key::KeyH) {
                if let Some(tile) = self.hovered_tile {
                    self.worker
                        .send(WorkerCommand::ChangeTile(tile.pos, ConnexNumber(-1)));
                }
            }

            if input.just_pressed(Key::KeyU) {
                if let Some(tile) = self.hovered_tile {
                    self.worker
                        .send(WorkerCommand::ChangeTile(tile.pos, Stability(0.1)));
                }
            }

            if input.just_pressed(Key::KeyJ) {
                if let Some(tile) = self.hovered_tile {
                    self.worker
                        .send(WorkerCommand::ChangeTile(tile.pos, Stability(-0.1)));
                }
            }

            if input.just_pressed(Key::KeyI) {
                if let Some(tile) = self.hovered_tile {
                    self.worker
                        .send(WorkerCommand::ChangeTile(tile.pos, Reactivity(0.1)));
                }
            }

            if input.just_pressed(Key::KeyK) {
                if let Some(tile) = self.hovered_tile {
                    self.worker
                        .send(WorkerCommand::ChangeTile(tile.pos, Reactivity(-0.1)));
                }
            }

            if input.just_pressed(Key::KeyO) {
                if let Some(tile) = self.hovered_tile {
                    self.worker
                        .send(WorkerCommand::ChangeTile(tile.pos, Energy(20.0)));
                }
            }

            if input.just_pressed(Key::KeyL) {
                if let Some(tile) = self.hovered_tile {
                    self.worker
                        .send(WorkerCommand::ChangeTile(tile.pos, Energy(-20.0)));
                }
            }

            if input.just_pressed(Key::KeyP) {
                if let Some(tile) = self.hovered_tile {
                    self.worker
                        .send(WorkerCommand::ChangeTile(tile.pos, Delta(1)));
                }
            }

            if input.just_pressed(Key::Semicolon) {
                if let Some(tile) = self.hovered_tile {
                    self.worker
                        .send(WorkerCommand::ChangeTile(tile.pos, Delta(-1)));
                }
            }

            if ainput.just_pressed(Action::Step) {
                self.worker.send(WorkerCommand::Step());
            }
        }

        if state.player.admin {
            if input.just_pressed(Key::KeyB) {
                state.player.creative = !state.player.creative;
                if !state.player.creative {
                    state.camera_scroll = state.camera_scroll.clamp(0.0, 30.0);
                    state.camera.scale = (state.camera_scroll * 0.1).exp();
                }
            }
        }

        if ainput.just_pressed(Action::Pause) {
            self.paused = !self.paused;
            self.worker.send(WorkerCommand::Pause(self.paused));
        }
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
