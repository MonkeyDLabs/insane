use super::custom::*;
use crate::error::Result;

use insane_core::config::InsaneConfig;
use insane_core::environment::Environment;
use std::process::exit;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("doctor").about("Validate and diagnose configurations.")
}

// Clean command implementation
pub async fn execute(_args: &ArgMatches, config: &InsaneConfig, _env: &Environment) -> Result<()> {
    // println!("doctor command {:?}", args);
    let mut should_exit = false;
    for (_, check) in crate::doctor::run_all(&config).await {
        if !should_exit && !check.valid() {
            should_exit = true;
        }
        println!("{check}");
    }
    if should_exit {
        exit(1);
    }

    Ok(())
}
