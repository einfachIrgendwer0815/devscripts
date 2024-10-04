# Devscripts

[![Release][version_img]][releases]
[![License][license_img]](#license)
[![crates.io][crate_img]][cratesio]
[![docs.rs][docs_img]][docsrs]

Devscripts is a linux command-line tool that makes it easy to run (shell) scripts
depending on the user or current working directory without ever having to
modify `PATH`. For more information, see [How to use](#how-to-use).

### Table of Contents

  - [Description](#devscripts)
  - [Installation](#installation)
  - [How to use](#how-to-use)
  - [Configuration](#configuration)
  - [Feature & Pull requests](#feature--pull-requests)
  - [Development and Contributing](#development-and-contributing)
  - [License](#license)


## Installation

Devscripts can be installed via `cargo install`, the installed executable will
be called `dev`:

Latest stable version:
```bash
cargo install devscripts
```

Latest development version:
```bash
cargo install --git https://github.com/einfachIrgendwer0815/devscripts
```

The installed executable is called `dev`:
```bash
dev --version
# devscripts <version>

dev --help
# <devscripts help page>
```


## How to use

### Adding scripts

To add a script that can be run via `devscripts`, follow these two steps:

>[!NOTE]
> These scripts do **not** have to be shell scripts. They could also be written
> in, for example, python. In fact, `devscripts` will run *any* executable
> that is available in one `devscripts` paths (listed below).

#### Write the script

Write your shell script and place it in of these locations
(ascending in priority):
  - `/usr/share/devscripts`       (system-wide scripts)
  - `/usr/local/share/devscripts` (system-wide scripts)
  - `~/.local/share/devscripts`   (user-specific scripts)
  - `<repo-root>/.devscripts"`    (repository-local scripts, `<repo-root>` is the root of a **git** worktree)

Scripts with same names in higher-priority directories will override those in
lower-priority directories.
Additionally, `devscripts` ignores file endings, so avoid having multiple
scripts with the same name in the same directory. In such a case, `devscripts`
will use the script it encounters first. However, there are no guarantees which
one that will be.

>[!NOTE]
> System-wide, user-wide and repository-local script directories are
> configurable. See [Configuration](#configuration) for more information.

#### Make it executable

Set the executable bit using `chmod`:
```bash
chmod +x /path/to/your/script
```


### Running scripts

To run any script in one of [`devscripts` directories](#write-the-script), run:
```bash
dev run <SCRIPT-NAME>

# say, there is a script at ~/.local/share/devscripts/hello-world
dev run hello-world
# will run that script
```

If you run `dev` **anywhere inside a git worktree**, the above mentioned
repository-local script directory will be searched for scripts as well. These
repository-local scripts do *not* need to be known by git.


## Configuration

Devscripts can be configured by creating/editing the following files
(ascending in priority):
  - `/etc/devscripts/config.toml`          (global/system-wide configuration)
  - `~/.config/devscripts/config.toml`     (user configuration)
  - `<repo-root>/.devscripts/.config.toml` (repository-local configuration)

Higher-priority configurations will override lower-priority ones.

The current default configuration looks like this:
```toml
[paths.scripts]
# Directories for system-wide scripts.
system = [
    "/usr/share/devscripts",
    "/usr/local/share/devscripts",
]

# Directories for user-specific scripts.
user = [
    "~/.local/share/devscripts",
]

# Directories for repository-local scripts. These paths
# are applied relative to a repository root.
repository = [
    "./.devscripts"
]

```


## Feature & Pull requests

Devscripts is currently in a very early stage of development, so all feature
requests and PRs are very much appreciated.


## Development and contributing

This project uses the [Conventional Commits] standard for commit messages with
the following additional rules:

  - Prefer `!` over `BREAKING CHANGE:`
  - Recommended commit types are:
      - `build`   : Changes that affect the build system or dependencies
      - `ci`      : Changes to CI configuration
      - `docs`    : Documentation only changes
      - `feat`    : New features or feature enhancements
      - `fix`     : Bug fixes
      - `refactor`: Changes that do not add a feature or fix a bug
      - `perf`    : Changes that improve performance
      - `style`   : Changes that keep the meaning of the code as-is (white-space changes, formatting, etc.)
      - `test`    : Adding/changing tests
      - `chore`   : repository-related stuff, version bumps, etc.

>[!NOTE]
> If you don't know how to use [Conventional Commits],
> don't worry. Just write your commit messages how you always do.


## License

Copyright 2024 einfachIrgendwer0815 and contributors.

Copyrights in this project are retained by contributors.
No copyright assignment is required to contribute to this project.

Except as otherwise noted (below and/or in individual files),
this project is licensed under either the Apache License,
Version 2.0 (LICENSE-APACHE), or the MIT license (LICENSE-MIT), at your option.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.


<!-- links !-->
[Conventional Commits]: https://www.conventionalcommits.org/en/v1.0.0/

[releases]: https://github.com/einfachIrgendwer0815/devscripts/releases
[cratesio]: https://crates.io/crates/devscripts
[docsrs]: https://docs.rs/devscripts

[version_img]: https://img.shields.io/github/v/release/einfachIrgendwer0815/devscripts?color=8800AA&style=flat-square&include_prereleases
[license_img]: https://img.shields.io/badge/license-MIT_OR_Apache--2.0-orange?style=flat-square
[crate_img]: https://img.shields.io/crates/v/devscripts?style=flat-square
[docs_img]: https://img.shields.io/docsrs/devscripts?style=flat-square&color=blue
