//! Devscripts configuration data structures

// attributes with types which do not implement
// `Eq` may be added to the configuration in the future.
#![allow(clippy::derive_partial_eq_without_eq)]

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[cfg(feature = "serde")]
mod reader;
#[cfg(feature = "serde")]
pub use reader::*;

/// Main configuration data structure for devscripts.
///
/// Use [`default()`](Self::default) to obtain the default configuration.
#[cfg_attr(
    feature = "serde",
    doc = "[`ConfigReader`] can be used to load configuration from the filesystem."
)]
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Config {
    /// Options specifying paths or lists of paths
    pub paths: Paths,
}

/// Options specifying paths or lists of paths
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Paths {
    /// Search paths for script files
    pub scripts: ScriptPaths,
}

/// Search paths for script files
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct ScriptPaths {
    /// List of directories containing system-wide scripts.
    ///
    /// (Default: `/usr/share/devscripts/scripts`, `/usr/local/share/devscripts/scripts`)
    pub system: Vec<PathBuf>,

    /// List of directories containing user-specific scripts.
    ///
    /// (Default: `~/.local/share/devscripts/scripts`)
    pub user: Vec<PathBuf>,

    /// List of directories containing repository-specific scripts.
    ///
    /// These paths are expected to be relative to the root directory of a git
    /// repository if the current working directory is located inside a git
    /// repository. If not inside of a git repository, this option is ignored.
    ///
    /// (Default: `./.devscripts/scripts`, `./.dev/scripts`)
    pub repository: Vec<PathBuf>,
}

impl Default for ScriptPaths {
    fn default() -> Self {
        Self {
            system: vec![
                "/usr/share/devscripts/scripts".into(),
                "/usr/local/share/devscripts/scripts".into(),
            ],
            user: vec!["~/.local/share/devscripts/scripts".into()],
            repository: vec!["./.devscripts/scripts".into(), "./.dev/scripts".into()],
        }
    }
}
