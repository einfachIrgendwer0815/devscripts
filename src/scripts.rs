//! Utilities for running and interacting with script files.

use std::fs;
use std::io;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::process::ExitStatus;

use thiserror::Error;

use crate::config::Config;
use crate::path;

/// Error type associated with the [`run()`] function.
#[derive(Debug, Error)]
pub enum RunError {
    /// An I/O operation failed.
    #[error("I/O operation failed: {0}")]
    Io(#[from] io::Error),

    /// Script could not be found. Contains the script's name.
    #[error("Script {0:?} could not be found.")]
    ScriptNotFound(String),
}

/// Run a certain script with additional `args`.
pub fn run(name: &str, config: &Config, args: &[&str]) -> Result<ExitStatus, RunError> {
    let script =
        find_script(name, config)?.ok_or_else(|| RunError::ScriptNotFound(name.to_string()))?;

    let mut child = Command::new(script).args(args).spawn()?;

    Ok(child.wait()?)
}

/// Returns whether a certain script exists. Returns `false` when
/// errors occur.
pub fn script_exists(name: &str, config: &Config) -> bool {
    find_script(name, config)
        .map(|path| path.is_some())
        .unwrap_or_default()
}

/// Search for a script file in the configured paths.
pub fn find_script(name: &str, config: &Config) -> Result<Option<PathBuf>, io::Error> {
    let dirs = path::directories_iter(config)?;

    for dir in dirs {
        match search_dir(name, &dir) {
            Ok(file) => {
                if file.is_some() {
                    return Ok(file);
                }
            }
            Err(e) => {
                if let ErrorKind::NotFound = e.kind() {
                    continue;
                } else {
                    return Err(e);
                }
            }
        }
    }

    Ok(None)
}

/// Search file by name in a specific directory
fn search_dir(name: &str, dir: &Path) -> Result<Option<PathBuf>, io::Error> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;

        if entry.file_type()?.is_file() && entry.file_name().to_string_lossy() == name {
            return Ok(Some(entry.path()));
        }
    }
    Ok(None)
}
