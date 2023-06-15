use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winit::event::VirtualKeyCode as Key;

#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Action {
    Exit,
    MoveUp,
    MoveLeft,
    MoveDown,
    MoveRight,
    Timers,
    TileInfo,
    TotalEnergy,
}

pub type Keybinds = HashMap<Action, Key>;

pub fn default_keybinds() -> Keybinds {
    HashMap::from([
        (Action::Exit, Key::Escape),
        (Action::MoveUp, Key::W),
        (Action::MoveLeft, Key::A),
        (Action::MoveDown, Key::S),
        (Action::MoveRight, Key::D),
        (Action::Timers, Key::T),
        (Action::TileInfo, Key::I),
        (Action::TotalEnergy, Key::E),
    ])
}

