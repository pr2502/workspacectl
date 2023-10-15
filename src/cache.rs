//! Read write the state from a global key-value store
//!
//! The store is a simple file store in the platform program cache (`~/.cache/workspacectl`), each
//! key maps to a file name and the value is the file's contents stripped of whitespace. Values
//! must always be valid UTF-8 and cannot containe newlines.

use std::fs;
use std::io::Write;
use std::path::PathBuf;

use anyhow::{Context, Result};
use atomicwrites::AtomicFile;

#[derive(Debug, Clone, Copy)]
pub enum Key {
    /// Currently open workspace
    Current,
}

impl Key {
    fn filename(&self) -> &'static str {
        match self {
            Key::Current => "current",
        }
    }
}

fn dir_path() -> Result<PathBuf> {
    let cache_dir = dirs::cache_dir().context("could not determine user cache directory")?;
    Ok(cache_dir.join("workspacectl"))
}

pub fn read(key: Key) -> Result<String> {
    let path = dir_path()?.join(key.filename());
    Ok(fs::read_to_string(&path)
        .with_context(|| format!("reading cache file at {path:?}"))?
        .trim()
        .to_owned())
}

pub fn write(key: Key, value: String) -> Result<()> {
    let path = dir_path()?;
    fs::create_dir_all(&path).with_context(|| format!("could not cache directory at {path:?}"))?;
    let path = path.join(key.filename());
    AtomicFile::new(&path, atomicwrites::AllowOverwrite)
        .write(|file| {
            file.write_all(value.trim().as_bytes())?;
            file.write_all(b"\n")
        })
        .with_context(|| format!("atomically write cache file at {path:?}"))
}
