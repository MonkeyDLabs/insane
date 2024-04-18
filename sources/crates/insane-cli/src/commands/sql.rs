use super::custom::*;
use crate::error::Result;

use insane_core::{config::InsaneConfig, hook::Hooks};
use insane_core::environment::Environment;

use clap::ArgAction;

use insane_core::sql::{connect, create, migrate, migrate_down, reset, status};
use sea_orm_migration::MigratorTrait;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("database")
        .about("Perform DB operations")
        .subcommand(Command::new("create").about("Create schema"))
        .subcommand(
            Command::new("migrate")
                .about("Migrate schema")
                .arg(
                    Arg::new("up")
                        .short('u')
                        .long("up")
                        .action(ArgAction::SetTrue)
                        .help("Apply all pending migrations"),
                )
                .arg(
                    Arg::new("down")
                        .short('d')
                        .long("down")
                        .num_args(1)
                        .help("Downgrade the database by n steps"),
                ),
        )
        .subcommand(Command::new("reset").about("Drop all tables, then reapply all migrations"))
        .subcommand(Command::new("status").about("Migration status"))
        .subcommand(
            Command::new("entities").about("Generate entity .rs files from database schema"),
        )
        .subcommand(Command::new("truncate").about("Truncate data in tables (without dropping)"))
}

// Clean command implementation
pub async fn execute<H: Hooks, M: MigratorTrait>(
    matches: &ArgMatches,
    config: &InsaneConfig,
    _env: &Environment,
) -> Result<()> {
    println!("database command {:?}", matches);

    match matches.subcommand() {
        Some(("create", _)) => create(&config.sql.uri).await?,
        Some(("migrate", _)) | Some(("reset", _)) | Some(("status", _)) | Some(("truncate", _)) => {
            println!("migrate, reset, status, truncate");
            let connection = connect(&config.sql).await?;
            match matches.subcommand() {
                Some(("migrate", args)) => {
                    let down_steps = args.get_one::<String>("down");
                    if let Some(steps) = down_steps {
                        let steps = steps.parse::<u32>().unwrap();
                        migrate_down::<M>(&connection, Some(steps)).await?
                    } else {
                        // by default apply all pending migrations
                        migrate::<M>(&connection).await?
                    }
                }
                Some(("reset", _)) => reset::<M>(&connection).await?,
                Some(("status", _)) => status::<M>(&connection).await?,
                Some(("truncate", _)) => H::truncate(&connection).await?,
                // Some(("entities", _)) => println!("entities"),
                _ => unreachable!(), // This should never happen due to the match arm patterns
            }
        }
        _ => println!("unknown"),
    }

    Ok(())
}
