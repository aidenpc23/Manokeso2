use std::collections::HashMap;

use ron::extensions::Extensions;
use serde::{Deserialize, Serialize};
use winit::event::VirtualKeyCode as Key;

use crate::keybinds::Action;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub keybinds: Option<HashMap<Action, Key>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            keybinds: None,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let ron = ron::Options::default().with_default_extension(Extensions::IMPLICIT_SOME);
        if let Some(path) = dirs::config_dir() {
            if let Ok(contents) = std::fs::read_to_string(path.join("manokeso/config.ron")) {
                match ron.from_str::<Config>(&contents) {
                    Ok(config) => {
                        return config
                    },
                    Err(err) => {
                        let line = contents.lines().nth(err.position.line - 1).unwrap_or("???");
                        println!("Failed to load config:");
                        println!("{:?}", err.code);
                        println!("{}", line);
                        println!("{}^", " ".repeat(err.position.col - 1));
                    }
                }
            }
        }
        Self::default()
    }
}
