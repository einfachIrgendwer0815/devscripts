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

mod completions {
    pub const BASH: &str = include_str!("../../../completions/devscripts.bash");
}

mod clap;

use std::os::unix::process::ExitStatusExt;
use std::process::ExitCode;

use ::clap::ArgMatches;
use devscripts::config::Config;
use devscripts::RunError;

fn main() -> Result<ExitCode, anyhow::Error> {
    let mut app = clap::build_clap_app();
    let matches = app.get_matches_mut();

    let config = Config::read()?;

    match matches.subcommand() {
        Some(("run", args)) => run(args, config),
        Some(("list-scripts", _)) => {
            let scripts = devscripts::all_scripts(&config)?;
            for script in scripts {
                println!("{script}");
            }
            Ok(ExitCode::SUCCESS)
        }
        Some(("completions", args)) => {
            #[expect(
                clippy::unwrap_used,
                reason = "`shell` is a required parameter, its existence is checked by clap."
            )]
            let shell = args.get_one::<String>("shell").unwrap();

            match shell.as_str() {
                "bash" => print!("{}", completions::BASH),
                _ => unreachable!("Argument `shell` has no other possible values."),
            }
            Ok(ExitCode::SUCCESS)
        }
        _ => {
            app.print_help()?;
            Ok(ExitCode::SUCCESS)
        }
    }
}

fn run(matches: &ArgMatches, config: Config) -> Result<ExitCode, anyhow::Error> {
    #[expect(
        clippy::unwrap_used,
        reason = "`script_name` is a required parameter, its existence is checked by clap."
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
