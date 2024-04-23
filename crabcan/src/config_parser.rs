use crate::errors::Errcode;

use serde::Deserialize;
use std::{fs, path::PathBuf};
use toml;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub debug: bool,
    pub uid: u32,
    pub mount_dir: PathBuf,
    pub command: String,
    pub additional_paths: Vec<String>,
}

pub fn load_config(config_file_path: PathBuf) -> Result<Config, Errcode> {
    let contents = fs::read_to_string(config_file_path)
        .map_err(|_| Errcode::ConfigFileError(0))?;
    let config: Config = toml::from_str(&contents)
        .map_err(|_| Errcode::ConfigFileError(1))?;
    Ok(config)
} 