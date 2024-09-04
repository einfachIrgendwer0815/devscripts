//! Path manipulation utilities

use std::borrow::Cow;
use std::env::current_dir;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use git2::{Repository, RepositoryOpenFlags};

/// Replaces `~` at the beginning of a path with
/// the current user's home directory.
pub fn extend_home(path: &Path) -> Cow<Path> {
    match path.strip_prefix("~") {
        Ok(p) => home::home_dir().unwrap_or_default().join(p).into(),
        Err(_) => path.into(),
    }
}

/// Find git root
pub fn git_root() -> Result<Option<PathBuf>, io::Error> {
    let Ok(repo) = Repository::open_ext(
        current_dir()?,
        RepositoryOpenFlags::empty(),
        &[] as &[&std::ffi::OsStr],
    ) else {
        return Ok(None);
    };

    Ok(repo.workdir().map(ToOwned::to_owned))
}
