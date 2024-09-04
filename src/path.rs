//! Path manipulation utilities

use std::borrow::Cow;
use std::path::Path;

/// Replaces `~` at the beginning of a path with
/// the current user's home directory.
pub fn extend_home(path: &Path) -> Cow<Path> {
    match path.strip_prefix("~") {
        Ok(p) => home::home_dir().unwrap_or_default().join(p).into(),
        Err(_) => path.into(),
    }
}
