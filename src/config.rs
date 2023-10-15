use std::fs;
use std::path::PathBuf;

pub mod data;
use anyhow::{Context, Result};
pub use data::*;

/// Returns path to the config directory
fn dir_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir().context("could not determine user config directory")?;
    Ok(config_dir.join("workspacectl"))
}

/// Returns path to the config file
fn config_path() -> Result<PathBuf> {
    Ok(dir_path()?.join("config.toml"))
}

pub fn read() -> Result<Config> {
    let path = config_path()?;
    let buf =
        fs::read_to_string(&path).with_context(|| format!("reading config file at {path:?}"))?;
    toml::from_str(&buf).with_context(|| format!("parsing config file at {path:?}"))
}
