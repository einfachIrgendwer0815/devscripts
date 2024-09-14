//! Path manipulation utilities

use std::borrow::Cow;
use std::env::current_dir;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use git2::{Repository, RepositoryOpenFlags};

use crate::config::Config;

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

/// Get an iterator over all configured script directories, in order from high to low priority.
pub fn directories_iter(config: &Config) -> Result<impl Iterator<Item = Cow<'_, Path>>, io::Error> {
    let repo_root = git_root()?;
    let user_dir_iter = config.paths.scripts.user.iter().map(|p| extend_home(p));
    let system_dir_iter = config.paths.scripts.system.iter().map(Cow::from);

    let mut dirs: Box<dyn Iterator<Item = Cow<'_, Path>>> = Box::new([].into_iter());

    if let Some(root) = repo_root {
        let repo_dir_iter = config
            .paths
            .scripts
            .repository
            .iter()
            .map(move |p| Cow::Owned(root.join(p)));
        dirs = Box::new(dirs.chain(repo_dir_iter));
    }

    dirs = Box::new(dirs.chain(user_dir_iter));
    dirs = Box::new(dirs.chain(system_dir_iter));

    Ok(dirs)
}
