use clap::Command;
use clap::{Arg, ArgAction};

pub fn build_clap_app() -> Command {
    Command::new("devtools")
        .bin_name("dev")
        .about("Run shell scripts conveniently.")
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .arg(
            Arg::new("script_name")
                .action(ArgAction::Set)
                .value_name("SCRIPT")
                .help("Name of the script to execute"),
        )
        .arg(
            Arg::new("args")
                .action(ArgAction::Append)
                .value_name("ARGS")
                .help("Arguments to be passed to the script")
                .trailing_var_arg(true)
                .allow_hyphen_values(true),
        )
}
