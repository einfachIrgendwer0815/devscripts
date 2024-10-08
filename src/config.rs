//! Devscripts configuration data structures

#![expect(
    clippy::derive_partial_eq_without_eq,
    reason = "Non-`Eq` typed attributes may be added \
              to the configuration in the future."
)]

#[cfg(feature = "serde")]
use figment::{
    providers::{Format, Serialized, Toml},
    Figment,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[cfg(feature = "serde")]
use crate::path::git_root;

/// Main configuration data structure for devscripts.
///
/// Use [`default()`](Self::default) to obtain the default configuration.
#[cfg_attr(
    feature = "serde",
    doc = "[`read()`](Self::read) and [`figment()`](Self::figment) can be used to obtain configuration from files."
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
    /// (Default: `/usr/share/devscripts`, `/usr/local/share/devscripts`)
    pub system: Vec<PathBuf>,

    /// List of directories containing user-specific scripts.
    ///
    /// (Default: `~/.local/share/devscripts`)
    pub user: Vec<PathBuf>,

    /// List of directories containing repository-specific scripts.
    ///
    /// These paths are expected to be relative to the root directory of a git
    /// repository if the current working directory is located inside a git
    /// repository. If not inside of a git repository, this option is ignored.
    ///
    /// (Default: `./.devscripts`)
    pub repository: Vec<PathBuf>,
}

impl Config {
    /// Figment for reading this config, provided with default files.
    ///
    /// Default configuration files:
    ///   - `/etc/devscripts/config.toml`
    ///   - `~/.config/devscripts/config.toml`
    ///   - `<repository-root>/.devscripts/.config.toml`.
    #[cfg(feature = "serde")]
    pub fn figment() -> Figment {
        let user_home = home::home_dir().unwrap_or_default();
        let git_root = git_root().unwrap_or_default();

        let mut figment = Figment::new()
            .merge(Serialized::defaults(Self::default()))
            .merge(Toml::file("/etc/devscripts/config.toml"))
            .merge(Toml::file(
                user_home.join("./.config/devscripts/config.toml"),
            ));

        if let Some(git_root) = git_root {
            figment = figment.merge(Toml::file(git_root.join("./.devscripts/.config.toml")));
        }

        figment
    }

    /// Read configuration from files.
    ///
    /// For a list of default configuration files,
    /// see [`figment()`](Self::figment).
    #[cfg(feature = "serde")]
    pub fn read() -> Result<Self, figment::Error> {
        Self::figment().extract()
    }
}

impl Default for ScriptPaths {
    fn default() -> Self {
        Self {
            system: vec![
                "/usr/share/devscripts".into(),
                "/usr/local/share/devscripts".into(),
            ],
            user: vec!["~/.local/share/devscripts".into()],
            repository: vec!["./.devscripts".into()],
        }
    }
}
