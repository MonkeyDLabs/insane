// use super::prelude::*;
// use crate::{configs::OcolConfig, environment::Environment, ocol::Ocol};
use insane_core::environment::Environment;

use super::custom::*;
use crate::error::Result;
use insane_core::{config::InsaneConfig, hook::Hooks};

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("version").about("Show the application version.")
}

// Clean command implementation
pub async fn execute<I: Hooks>(
    _args: &ArgMatches,
    _config: &InsaneConfig,
    _env: &Environment,
) -> Result<()> {
    println!("{}", I::app_version());
    Ok(())
}
