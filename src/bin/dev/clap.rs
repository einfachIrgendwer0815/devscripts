use clap::Command;

pub fn build_clap_app() -> Command {
    Command::new("devtools")
        .bin_name("dev")
        .about("Run shell scripts conveniently.")
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
}
