use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

pub mod data;
use anyhow::{Context, Result};
pub use data::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use toml::map::Entry;
use toml::{Table, Value};

/// Returns path to the config directory
fn dir_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir().context("could not determine user config directory")?;
    Ok(config_dir.join("workspacectl"))
}

/// Returns path to the config file
fn config_path() -> Result<PathBuf> {
    Ok(dir_path()?.join("config.toml"))
}

pub fn read() -> Result<Option<Config>> {
    let path = config_path()?;
    let buf = match fs::read_to_string(&path) {
        Ok(buf) => buf,
        Err(err) if err.kind() == ErrorKind::NotFound => return Ok(None),
        Err(err) => {
            return Err(err).with_context(|| format!("reading config file at {path:?}"))?;
        }
    };
    toml::from_str(&buf)
        .with_context(|| format!("parsing config file at {path:?}"))
        .map(Some)
}

/// Reads the global config and fills in missing keys from it
pub fn fill_defaults<T>(config: T) -> Result<T>
where
    T: Serialize + DeserializeOwned,
{
    let Some(defaults) = read()? else {
        return Ok(config);
    };

    let defaults = toml::Value::try_from(defaults).context("convert defaults to toml Value")?;
    let mut config = toml::Value::try_from(config).context("convert T to toml Value")?;

    fill_defaults_value(&mut config, defaults);

    config.try_into().context("convert merged back into T")
}

fn fill_defaults_value(config: &mut Value, defaults: Value) {
    match (config, defaults) {
        (Value::Table(config), Value::Table(defaults)) => fill_defaults_table(config, defaults),
        _ => {} // Only tables get merged, for anything else the config is left intact.
    }
}

fn fill_defaults_table(config: &mut Table, defaults: Table) {
    for (key, value) in defaults.into_iter() {
        match config.entry(key) {
            Entry::Vacant(e) => {
                e.insert(value);
            }
            Entry::Occupied(mut e) => {
                fill_defaults_value(e.get_mut(), value);
            }
        }
    }
}
