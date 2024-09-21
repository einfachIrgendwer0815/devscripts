use clap::Command;
use clap::{Arg, ArgAction};

pub fn build_clap_app() -> Command {
    Command::new("devscripts")
        .bin_name("dev")
        .about("Run shell scripts conveniently.")
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .subcommand(
            Command::new("list-scripts").about("List available scripts in alphabetical order."),
        )
        .subcommand(
            Command::new("run")
                .about("Run a script")
                .arg(
                    Arg::new("script_name")
                        .action(ArgAction::Set)
                        .value_name("SCRIPT")
                        .help("Name of the script to execute")
                        .required(true),
                )
                .arg(
                    Arg::new("args")
                        .action(ArgAction::Append)
                        .value_name("ARGS")
                        .help("Arguments to be passed to the script")
                        .trailing_var_arg(true)
                        .allow_hyphen_values(true),
                ),
        )
}
