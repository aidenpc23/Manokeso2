use std::{
    fs::{File, create_dir_all},
    io::{Read, Write}, path::PathBuf,
};

use crate::rsc::GAME_NAME;

pub fn save<T: serde::Serialize>(name: &str, data: &T) -> Result<(), SaveError> {
    let dir = save_dir();
    create_dir_all(dir.clone()).map_err(|e| SaveError::CreateDir(e))?;
    let mut file = File::create(dir.join(name)).map_err(|e| SaveError::CreateFile(e))?;
    let encoded: Vec<u8> = bincode::serialize(data).map_err(|e| SaveError::Serialize(e))?;
    file.write_all(&encoded).map_err(|e| SaveError::WriteFile(e))?;
    Ok(())
}

pub fn load<T: serde::de::DeserializeOwned>(name: &str) -> Result<T, LoadError> {
    let mut file = File::open(save_dir().join(name)).map_err(|e| LoadError::OpenFile(e))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| LoadError::ReadFile(e))?;
    let data = bincode::deserialize(&buffer).map_err(|e| LoadError::Deserialize(e))?;
    Ok(data)
}

fn save_dir() -> PathBuf {
    if let Some(dir) = dirs::data_dir() {
        dir.join(GAME_NAME).join("saves")
    } else {
        PathBuf::from(GAME_NAME).join("saves")
    }
}

#[derive(Debug)]
pub enum SaveError {
    Serialize(bincode::Error),
    CreateDir(std::io::Error),
    CreateFile(std::io::Error),
    WriteFile(std::io::Error),
}

#[derive(Debug)]
pub enum LoadError {
    OpenFile(std::io::Error),
    ReadFile(std::io::Error),
    Deserialize(bincode::Error),
}
