use crate::error::Result;
pub use clap::{arg, Arg, ArgMatches, Command};

use insane_core::config::InsaneConfig;
use insane_core::environment::Environment;

#[async_trait::async_trait]
pub trait CommandCustom {
    fn name(&self) -> &str;
    fn make_subcommand(&self) -> Command;
    async fn execute(
        &self,
        args: &ArgMatches,
        config: &InsaneConfig,
        env: &Environment,
    ) -> Result<()>;
}
