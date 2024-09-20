//! Utilities for running and interacting with script files.

use std::collections::HashSet;
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
///
/// This function spawns a new process that executes the script with
/// the given `args`. It will block until that process exits.
pub fn run(name: &str, config: &Config, args: &[&str]) -> Result<ExitStatus, RunError> {
    let script =
        find_script(name, config)?.ok_or_else(|| RunError::ScriptNotFound(name.to_string()))?;

    let mut child = Command::new(script).args(args).spawn()?;

    Ok(child.wait()?)
}

/// Returns whether a certain script exists. Returns `false` when
/// errors occur.
///
/// If you want to get error information in case of an error,
/// use [`find_script()`] instead.
///
/// These are equivalent:
/// ```no_run
/// use devscripts::{script_exists, find_script};
/// use devscripts::Config;
///
/// let config = Config::default();
///
/// let a = script_exists("my_script", &config);
/// let b = find_script("my_script", &config).ok().flatten().is_some();
/// assert_eq!(a, b);
/// ```
#[inline]
pub fn script_exists(name: &str, config: &Config) -> bool {
    find_script(name, config).ok().flatten().is_some()
}

/// Search for a script file in the configured paths and return its path if found.
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

/// Get names of all available scripts.
///
/// Note: the returned values are script *names* not paths.
pub fn all_scripts(config: &Config) -> Result<Vec<String>, io::Error> {
    let mut set = HashSet::new();

    let dirs = path::directories_iter(config)?;

    for dir in dirs {
        let read_dir = match fs::read_dir(dir) {
            Ok(rd) => rd,
            Err(e) => {
                if let ErrorKind::NotFound = e.kind() {
                    continue;
                } else {
                    return Err(e);
                }
            }
        };

        for entry in read_dir {
            let entry = entry?;

            if entry.file_type()?.is_file() {
                let file_name = entry.file_name().to_string_lossy().into_owned();
                set.insert(file_name);
            }
        }
    }

    let mut scripts = set.drain().collect::<Vec<_>>();
    scripts.sort();

    Ok(scripts)
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
