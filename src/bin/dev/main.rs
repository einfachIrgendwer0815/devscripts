#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![warn(clippy::too_many_lines)]
// `allow_attributes_without_reason` requires the feature `lint_reason`.
// Activating this lint should be reconsidered once `lint_reason` is stabilized.
// #![warn(clippy::allow_attributes_without_reason)]
#![warn(clippy::use_debug)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]
#![cfg_attr(not(debug_assertions), deny(clippy::todo))]
#![cfg_attr(not(debug_assertions), deny(clippy::unimplemented))]

mod clap;

use std::os::unix::process::ExitStatusExt;
use std::process::ExitCode;

use devtools::RunError;

fn main() -> Result<ExitCode, anyhow::Error> {
    let mut app = clap::build_clap_app();
    let matches = app.get_matches_mut();

    if !matches.contains_id("script_name") {
        app.print_help()?;
        return Ok(ExitCode::SUCCESS);
    }

    let config = devtools::config::ConfigReader::with_default_paths().read()?;

    #[allow(clippy::unwrap_used)] // That this argument exists was already checked.
    let script_name = matches.get_one::<String>("script_name").unwrap();
    let args = matches
        .get_many::<String>("args")
        .unwrap_or_default()
        .collect::<Vec<_>>();
    let args_borrowed = args.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    let status = devtools::run(script_name, &config, &args_borrowed);
    match status {
        Ok(status) => match status.code() {
            Some(code) => Ok(ExitCode::from(code as u8)),
            None => Ok(ExitCode::from(
                128 + status.signal().unwrap_or_default() as u8,
            )),
        },
        Err(e) => {
            eprintln!("{e}");

            match e {
                RunError::Io(_) => Ok(ExitCode::FAILURE),
                RunError::ScriptNotFound(_) => Ok(ExitCode::from(127)),
            }
        }
    }
}
