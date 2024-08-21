mod clap;

fn main() -> Result<(), anyhow::Error> {
    let mut app = clap::build_clap_app();
    let matches = app.get_matches_mut();

    if !matches.args_present() {
        app.print_help()?;
    }

    Ok(())
}
