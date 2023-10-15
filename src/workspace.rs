//! Read and write the workspace definition database
//!
//! The database is located in the platform configuration directory for `workspacectl`. For example
//! `~/.config/workspacectl` on Linux.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{ensure, Context, Result};
use atomicwrites::AtomicFile;
use log::{error, info, warn};

mod data;
pub use data::*;
use walkdir::WalkDir;

use crate::cache::{self, Key};

/// Returns path to the directory used to store workspace definition files
fn dir_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir().context("could not determine user config directory")?;
    Ok(config_dir.join("workspacectl"))
}

/// Characters forbidden in workspace names
///
/// These are characters forbidden in *nix and windows file names and `.`, notably allowing `/` and
/// `\` because workspace configs can be organized into directories.
const FORBIDDEN_CHARACTERS: &[char] = &[
    '\n', '\r', '\t', // Special whitespace characters
    '\0', // Linux forbidden characters
    '<', '>', ':', '"', '|', '?', '*', // Windows forbidden characters
];

/// Returns path to the file used to store a particular workspace definition
///
/// Checks all the preconditions for workspace name
fn file_path(name: &str) -> Result<PathBuf> {
    ensure!(
        !name.contains(|ch: char| ch.is_ascii_control()),
        "workspace name cannot contain ascii control characters",
    );
    ensure!(
        !name.contains(FORBIDDEN_CHARACTERS),
        "workspace name cannot contain {FORBIDDEN_CHARACTERS:?}",
    );
    let name = Path::new(name);
    ensure!(
        name.is_relative(),
        "workspace name must be a relative path, got {name:?}",
    );
    let dir = dir_path()?;
    Ok(dir.join(name).with_extension("toml"))
}

/// Read workspace definition for workspace with name `name`
pub fn read(name: &str) -> Result<Workspace> {
    let path = Path::new(name).with_extension("toml");
    ensure!(
        path.is_relative(),
        "workspace name must be a relative path, got {name:?}",
    );
    let path = dir_path()?.join(path);
    let buf =
        fs::read_to_string(&path).with_context(|| format!("reading workspace file at {path:?}"))?;
    let mut workspace = toml::from_str::<Workspace>(&buf)
        .with_context(|| format!("parsing workspace file at {path:?}"))?;
    // Overwrite the `String::default()` generated by serde.
    workspace.name.push_str(name);
    Ok(workspace)
}

/// Create a new workspace definition
pub fn create(workspace: &Workspace) -> Result<()> {
    let path = file_path(&workspace.name)?;

    // Create parent directory when we are creating a new workspace.
    let parent = path.parent().unwrap_or_else(|| {
        panic!("workspace file path should always have a parent.\npath={path:?}\n")
    });
    fs::create_dir_all(parent)
        .with_context(|| format!("could not create parent directory for workspace at {path:?}"))?;

    let buf = toml::to_string_pretty(workspace).unwrap_or_else(|error| {
        panic!("workspace config should always be serializable but it wasn't.\nerror={error}\nconfig={workspace:#?}\n")
    });
    AtomicFile::new(&path, atomicwrites::DisallowOverwrite)
        .write(|file| file.write_all(buf.as_bytes()))
        .with_context(|| format!("atomically write workspace file at {path:?}"))
}

/// List all workspace definitions
///
/// List is sorted by file name.
pub fn list() -> Vec<String> {
    let dir = match dir_path() {
        Ok(dir) => dir,
        Err(err) => {
            error!("error reading workspace list: {err}");
            return Vec::new();
        }
    };
    WalkDir::new(&dir)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|entry| {
            // Filter out invalid names of directories recursively
            entry
                .file_name()
                .to_str()
                .map(|name| {
                    if name.contains(|ch: char| ch.is_ascii_control()) {
                        info!(
                            "ignoring path with ascii control characters {:?}",
                            entry.path(),
                        );
                        return false;
                    }
                    if name.contains(FORBIDDEN_CHARACTERS) {
                        info!(
                            "ignoring path with forbidden characters {:?} {:?}",
                            FORBIDDEN_CHARACTERS,
                            entry.path(),
                        );
                        return false;
                    }
                    true
                })
                .unwrap_or_else(|| {
                    info!(
                        "ignoring path with invalid UTF-8 characters {:?}",
                        entry.path(),
                    );
                    false
                })
        })
        .filter_map(|res| match res {
            // Filter out IO errors
            Ok(entry) => Some(entry),
            Err(err) => {
                warn!("encountered an error while gathering workspace list: {err}");
                None
            }
        })
        .filter(|entry| entry.path().is_file())
        .filter_map(|entry| {
            entry
                .path()
                .strip_prefix(&dir)
                .expect("all files must be within the base directory")
                .to_str()
                .and_then(|name| name.strip_suffix(".toml"))
                .map(|s| s.to_owned())
        })
        .collect()
}

pub fn current() -> Result<Workspace> {
    let name = cache::read(Key::Current).context("get current workspace name")?;
    read(&name).context("read current workspace definition")
}
