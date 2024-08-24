//! Utilities for reading [configuration](super::Config) from the filesystem

use std::borrow::Cow;
use std::io;
use std::path::Path;

use config::{Config as Cfg, File, FileFormat};
use thiserror::Error;

use super::Config;

/// Helper struct to read [`Config`] with priority levels (i.e. from multiple files).
///
/// It is recommended to use [`with_default_paths()`](Self::with_default_paths) to
/// obtain a new instance.
///
/// # Examples
///
/// Read configuration from the default configuration paths (see [`with_default_paths()`](Self::with_default_paths)).
/// ```no_run
/// use devtools::config::ConfigReader;
/// # use devtools::config::ReaderError;
/// # fn main() -> Result<(), ReaderError> {
///
/// let config = ConfigReader::with_default_paths().read()?;
///
/// # Ok(()) }
/// ```
///
/// Read default paths and an additional custom path.
/// ```no_run
/// use devtools::config::ConfigReader;
/// # use devtools::config::ReaderError;
/// # fn main() -> Result<(), ReaderError> {
///
/// let config = ConfigReader::with_default_paths()
///     .add_path("my_custom_config.toml", true)
///     .read()?;
///
/// # Ok(())
/// # }
/// ```
///
/// Read custom paths only.
/// ```no_run
/// use devtools::config::ConfigReader;
/// # use devtools::config::ReaderError;
/// # fn main() -> Result<(), ReaderError> {
///
/// let config = ConfigReader::new()
///     .add_path("~/custom_path_1.toml", true)
///     .add_path("custom_devtools_config_2.toml", false)
///     .read()?;
///
/// # Ok(()) }
pub struct ConfigReader<'a> {
    /// List of configured paths in the form `(path, required)`. If `required == true`,
    /// [`read()`](Self::read) will return an error if the file does not exist.
    paths: Vec<(Cow<'a, Path>, bool)>,
}

impl<'a> ConfigReader<'a> {
    /// Obtain a new instance without any file paths configured. Use
    /// [`add_path()`](Self::add_path) to add paths.
    pub fn new() -> Self {
        Self { paths: vec![] }
    }

    /// Creates a new instance with default configuration file paths configured.
    /// Default files:
    ///   - `/etc/devtools/config.toml`
    ///   - `~/.config/devtools/config.toml`
    ///   - `./.devtools/config.toml`.
    ///
    /// None of those files are required.
    /// No error will be returned if any of them does not exist.
    pub fn with_default_paths() -> Self {
        let user_home = home::home_dir().unwrap_or_default();

        Self {
            paths: vec![
                (Cow::Borrowed(Path::new("/etc/devtools/config.toml")), false),
                (
                    Cow::Owned(user_home.join("./.config/devtools/config.toml")),
                    false,
                ),
                (Cow::Borrowed(Path::new("./.devtools/config.toml")), false),
            ],
        }
    }

    /// Add configuration file by path. The order in which files are added is
    /// significant, later added files will override earlier added files if their
    /// configuration values overlap.
    ///
    /// If `required == true`, [`read()`](Self::read) will return an error if
    /// the file does not exist.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use devtools::config::ConfigReader;
    /// # use devtools::config::ReaderError;
    /// # fn main() -> Result<(), ReaderError> {
    ///
    /// let config = ConfigReader::new()
    ///     .add_path("a.toml", false)
    ///     .add_path("b.toml", false)
    ///     .read()?;
    ///
    /// // Values in b.toml will override those in a.toml.
    ///
    /// # Ok(()) }
    /// ```
    pub fn add_path<P: AsRef<Path> + ?Sized>(mut self, path: &'a P, required: bool) -> Self {
        self.paths.push((Cow::Borrowed(path.as_ref()), required));
        self
    }

    /// Reads all specified files and builds an instance of [`Config`].
    ///
    /// # Errors
    ///
    /// Will return an error if:
    ///   - any file configured as required does not exist
    ///   - any file is not in toml format
    ///   - an I/O operation fails
    pub fn read(self) -> Result<Config, ReaderError> {
        let mut builder = Cfg::builder();

        for (path, required) in self.paths {
            let exists = path.try_exists()?;
            let path_as_str = path.as_ref().to_string_lossy();

            if !exists {
                if !required {
                    continue;
                }

                return Err(ReaderError::FileNotFound(path_as_str.to_string()));
            }

            let source = File::new(&path_as_str, FileFormat::Toml);
            builder = builder.add_source(source);
        }

        Ok(builder.build()?.try_deserialize::<Config>()?)
    }
}

/// The error type associated with [`ConfigReader`].
#[derive(Debug, Error)]
pub enum ReaderError {
    /// Building configuration failed.
    #[error(transparent)]
    ConfigBuilder(#[from] config::ConfigError),

    /// A required file does not exist. Contains the file path.
    #[error("required file does not exist: {0}")]
    FileNotFound(String),

    /// An I/O operation failed.
    #[error("I/O operation failed: {0}")]
    Io(#[from] io::Error),
}

impl<'a> Default for ConfigReader<'a> {
    /// Returns the same as [`with_default_paths()`](Self::with_default_paths).
    #[inline]
    fn default() -> Self {
        Self::with_default_paths()
    }
}
