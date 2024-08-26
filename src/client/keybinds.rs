use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winit::keyboard::KeyCode as Key;

#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Action {
    Exit,
    MoveUp,
    MoveLeft,
    MoveDown,
    MoveRight,
    Pause,
    Step,
}

pub type Keybinds = HashMap<Action, Key>;

pub fn default_keybinds() -> Keybinds {
    HashMap::from([
        (Action::Exit, Key::Escape),
        (Action::MoveUp, Key::KeyW),
        (Action::MoveLeft, Key::KeyA),
        (Action::MoveDown, Key::KeyS),
        (Action::MoveRight, Key::KeyD),
        (Action::Pause, Key::Space),
        (Action::Step, Key::KeyX),
    ])
}

