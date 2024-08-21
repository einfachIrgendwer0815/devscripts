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

fn main() -> Result<(), anyhow::Error> {
    let mut app = clap::build_clap_app();
    let matches = app.get_matches_mut();

    if !matches.args_present() {
        app.print_help()?;
    }

    Ok(())
}
