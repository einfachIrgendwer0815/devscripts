# Changelog

## [0.1.0-alpha.2] - 2024-10-04

### Changed

  - **Breaking:** Use subcommand `dev run <script>` to run scripts instead of `dev <script>` ([8787d88])
  - **Breaking:** Change option `--list-scripts` to subcommand `list-scripts` ([b2c2db6])
  - **Breaking:** Ignore file endings ([285ee36])
  - **Breaking:** Rename cargo package from `dev-scripts` to `devscripts` ([023a073])
  - **Breaking:** Rename default repository-local config from `.devscripts/config.toml` to `.devscripts/.config.toml` ([ba5ee1a])
  - Improve documentation for library functions ([34d91aa])

### Added

  - Add shell completions for bash ([ab1a329])

### Removed

  - **Breaking:** Remove repository-local script path `.dev/scripts` from default ([a4d4d50])
  - **Breaking:** Remove `scripts` subdirectory from default script paths ([ba5ee1a])

### Fixed

  - Search repository-local config only at repository root ([9695439])


## [0.1.0-alpha.1] - 2024-09-19

_This is the first pre-release of devscripts, containing the basic functionality._

### Added

  - Add running system-wide, user-wide and repository-local shell scripts ([f94e095], [b18ff8b])
  - Add option to list all available scripts ([959a388])
  - Add configuration options for script paths ([0cf4cc5], [7247928])
  - Set MSRV to `1.81.0` ([ee1795b])

[0.1.0-alpha.2]: https://github.com/einfachIrgendwer0815/devscripts/releases/tag/v0.1.0-alpha.2
[8787d88]: https://github.com/einfachIrgendwer0815/devscripts/commit/8787d886ce8003899e957f05a2b57ee03169a262
[b2c2db6]: https://github.com/einfachIrgendwer0815/devscripts/commit/b2c2db62488e984f3a2ad837bcb82a4e23a64bf0
[285ee36]: https://github.com/einfachIrgendwer0815/devscripts/commit/285ee3687c2b0eae265a86ba8afe7cfdf4079319
[023a073]: https://github.com/einfachIrgendwer0815/devscripts/commit/023a07361663df71f0eb6be5805f42cbea711cad
[ba5ee1a]: https://github.com/einfachIrgendwer0815/devscripts/commit/ba5ee1ae87a6bdaf1bb70cdf8f2caee2905353d0
[34d91aa]: https://github.com/einfachIrgendwer0815/devscripts/commit/34d91aa49ff208b5df4d948b693c14dff3ee6c7f
[ab1a329]: https://github.com/einfachIrgendwer0815/devscripts/commit/ab1a3297ba09847db99d9dcf4879685292ced52a
[a4d4d50]: https://github.com/einfachIrgendwer0815/devscripts/commit/a4d4d504e07eb4bc34865b4d37ebc6c54270f0c2
[9695439]: https://github.com/einfachIrgendwer0815/devscripts/commit/96954398c73067ae581ca3ea754ae5b0bf203a0d

[0.1.0-alpha.1]: https://github.com/einfachIrgendwer0815/devscripts/releases/tag/v0.1.0-alpha.1
[f94e095]: https://github.com/einfachIrgendwer0815/devscripts/commit/f94e0959fc8aa2082417757ef9b86a8c6362e55a
[b18ff8b]: https://github.com/einfachIrgendwer0815/devscripts/commit/b18ff8ba07fee5459c8f6e50dcbc2f14028a7c1b
[959a388]: https://github.com/einfachIrgendwer0815/devscripts/commit/959a388335b0de4730841a4d0151d0947e717b97
[0cf4cc5]: https://github.com/einfachIrgendwer0815/devscripts/commit/0cf4cc5b2d368c43718027272896cf99fa7a6ef8 
[7247928]: https://github.com/einfachIrgendwer0815/devscripts/commit/72479282cf01e7c9b806f6ca146adc8b0414713b
[ee1795b]: https://github.com/einfachIrgendwer0815/devscripts/commit/ee1795b3310ab6438e9eeb07530d923564f775cd
