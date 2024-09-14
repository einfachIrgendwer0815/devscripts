#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![warn(clippy::too_many_lines)]
#![warn(clippy::allow_attributes)]
#![warn(clippy::use_debug)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]
#![cfg_attr(not(debug_assertions), deny(clippy::todo))]
#![cfg_attr(not(debug_assertions), deny(clippy::unimplemented))]

mod clap;

use std::os::unix::process::ExitStatusExt;
use std::process::ExitCode;

use ::clap::ArgMatches;
use devscripts::config::Config;
use devscripts::RunError;

fn main() -> Result<ExitCode, anyhow::Error> {
    let mut app = clap::build_clap_app();
    let matches = app.get_matches_mut();

    let config = devscripts::config::ConfigReader::with_default_paths().read()?;

    if matches.get_flag("list-scripts") {
        let scripts = devscripts::all_scripts(&config)?;
        for script in scripts {
            println!("{script}");
        }

        Ok(ExitCode::SUCCESS)
    } else {
        if !matches.contains_id("script_name") {
            app.print_help()?;
            return Ok(ExitCode::SUCCESS);
        }

        run(matches, config)
    }
}

fn run(matches: ArgMatches, config: Config) -> Result<ExitCode, anyhow::Error> {
    #[expect(
        clippy::unwrap_used,
        reason = "That `script_name` exists was checked above."
    )]
    let script_name = matches.get_one::<String>("script_name").unwrap();
    let args = matches
        .get_many::<String>("args")
        .unwrap_or_default()
        .collect::<Vec<_>>();
    let args_borrowed = args.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    let status = devscripts::run(script_name, &config, &args_borrowed);
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
