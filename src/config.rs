//! Devtools configuration data structures

// attributes with types which do not implement
// `Eq` may be added to the configuration in the future.
#![allow(clippy::derive_partial_eq_without_eq)]

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration data structure for devtools.
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
    /// (Default: `/usr/share/devtools/scripts`, `/usr/local/share/devtools/scripts`)
    pub system: Vec<PathBuf>,

    /// List of directories containing user-specific scripts.
    ///
    /// (Default: `~/.local/share/devtools/scripts`)
    pub user: Vec<PathBuf>,

    /// List of directories containing repository-specific scripts.
    ///
    /// These paths are expected to be relative to the root directory of a git
    /// repository if the current working directory is located inside a git
    /// repository. If not inside of a git repository, this option is ignored.
    ///
    /// (Default: `./.devtools/scripts`, `./.dev/scripts`)
    pub repository: Vec<PathBuf>,
}

impl Default for ScriptPaths {
    fn default() -> Self {
        Self {
            system: vec![
                "/usr/share/devtools/scripts".into(),
                "/usr/local/share/devtools/scripts".into(),
            ],
            user: vec!["~/.local/share/devtools/scripts".into()],
            repository: vec!["./.devtools/scripts".into(), "./.dev/scripts".into()],
        }
    }
}
