// use super::command_prelude::*;
// use crate::get_book_dir;
// use anyhow::Context;

use super::custom::*;

use clap::builder::NonEmptyStringValueParser;
use clap::ArgAction;
use insane_core::sql;
use insane_core::environment::Environment;

use crate::error::Result;
use insane_core::boot_loader::{boot_app, create_context};
use insane_core::{config::InsaneConfig, hook::Hooks};

#[cfg(feature = "with-sql")]
use sea_orm_migration::MigratorTrait;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("start")
        .about("Start about...")
        .arg(
            Arg::new("http")
                .long("http")
                .action(ArgAction::SetTrue)
                .help("Start HTTP server"),
        )
        .arg(
            Arg::new("grpc")
                .long("grpc")
                .action(ArgAction::SetTrue)
                .help("Start GRPC server"),
        )
        .arg(
            Arg::new("binding")
                .short('b')
                .long("binding")
                .num_args(1)
                .default_value("127.0.0.1")
                .value_parser(NonEmptyStringValueParser::new())
                .help("binding to listen on for HTTP connections"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .num_args(1)
                .default_value("3000")
                .value_parser(NonEmptyStringValueParser::new())
                .help("Port to use for HTTP connections"),
        )
}

#[cfg(feature = "with-sql")]
pub async fn execute<H: Hooks, M: MigratorTrait>(
    args: &ArgMatches,
    config: &InsaneConfig,
    environment: &Environment,
) -> Result<()> {
    let default_context = create_context::<H>(environment, config).await?;
    sql::prepare::<H, M>(&default_context.sql(), &config.sql).await?;
    boot_app::<H>(default_context).await?;
    Ok(())
}

#[cfg(not(feature = "with-sql"))]
pub async fn execute<H: Hooks>(
    args: &ArgMatches,
    config: &InsaneConfig,
    environment: &Environment,
) -> Result<()> {
    let default_context = create_context::<H>(environment, config).await?;
    boot_app::<H>(default_context).await?;
    Ok(())
}
