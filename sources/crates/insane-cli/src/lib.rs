pub mod error;
pub mod commands;
pub(crate) mod doctor;

use std::collections::HashMap;

#[cfg(feature = "with-sql")]
use sea_orm_migration::MigratorTrait;
// #[cfg(feature = "with-sql")]
// use insane_database::hook::DatabaseHooks;

use crate::commands::custom::CommandCustom;
use insane_core::environment::{DEFAULT_ENVIRONMENT, resolve_from_env, Environment};
use insane_core::{
    config::InsaneConfig,
    hook::Hooks,
    traces
};
use clap::{Arg, ArgMatches, Command};
use clap_complete::Shell;

const VERSION: &str = concat!("v", clap::crate_version!());

pub struct InsaneCli {
    custom_commands: HashMap<String, Box<dyn CommandCustom>>,
}

impl InsaneCli {
    pub fn new() -> Self {
        Self {
            custom_commands: HashMap::new(),
        }
    }

    pub fn add_custom_command<C: CommandCustom + 'static>(&mut self, command: C) {
        self.custom_commands
            .insert(command.name().to_string(), Box::new(command));
    }
    pub fn pre_run<H: Hooks>(&self) -> eyre::Result<(Environment, InsaneConfig, ArgMatches)> {
        let command = self.create_clap_command();
        let matches = command.get_matches();

        let environment: Environment = matches
            .get_one::<String>("environment")
            .map(|e| e.to_string())
            .unwrap_or_else(resolve_from_env)
            .into();

        let config = InsaneConfig::load_from_env::<H>(&environment)?;
        // let config = environment.load::<I>()?;

        if !H::init_logger(&config, &environment)? {
            traces::init(&config.tracing)?;
        }

        Ok((environment, config, matches))
    }

    pub async fn common_cli<I: Hooks>(
        &self,
        environment: &Environment,
        config: &InsaneConfig,
        matches: &ArgMatches,
    ) -> eyre::Result<Option<bool>> {
        let subcommand = matches.subcommand();
        let res = match subcommand {
            Some(("doctor", sub_matches)) => {
                let _ = commands::doctor::execute(sub_matches, &config, &environment).await?;
                Ok(Some(true))
            }
            Some(("version", sub_matches)) => {
                let _ = commands::version::execute::<I>(sub_matches, &config, &environment).await?;
                Ok(Some(true))
            }

            Some(("completions", sub_matches)) => (|| {
                let shell = sub_matches
                    .get_one::<Shell>("shell")
                    .ok_or_else(|| eyre::eyre!("Shell name missing."))?;

                let mut complete_app = self.create_clap_command();
                clap_complete::generate(
                    *shell,
                    &mut complete_app,
                    "ocol",
                    &mut std::io::stdout().lock(),
                );
                Ok(Some(true))
            })(),
            _ => Ok(None),
        };

        if let Err(e) = res {
            // utils::log_backtrace(&e);
            eprintln!("Error: {}", e);

            std::process::exit(101);
        }

        res
    }

    /// Create a list of valid arguments and sub-commands
    pub fn create_clap_command(&self) -> Command {
        let mut app = Command::new(clap::crate_name!())
            .about(clap::crate_description!())
            .author("Monkey d Teams")
            .version(VERSION)
            .propagate_version(true)
            .arg_required_else_help(true)
            // .after_help(
            //     "For more information about a specific command, try `mdbook <command> --help`\n\
            //   The source code for mdBook is available at: https://github.com/rust-lang/mdBook",
            // )
            .arg(
                Arg::new("environment")
                    .short('e')
                    .long("environment")
                    .global(true)
                    .value_name("ENVIRONMENT")
                    .default_value(DEFAULT_ENVIRONMENT)
                    .help("Specify the environment"),
            )
            .subcommand(commands::start::make_subcommand())
            .subcommand(commands::doctor::make_subcommand())
            .subcommand(commands::version::make_subcommand());

        #[cfg(feature = "with-sql")]
        {
            app = app.subcommand(commands::sql::make_subcommand());
        }

        for (_, command) in &self.custom_commands {
            app = app.subcommand(command.make_subcommand());
        }

        app
    }

    #[cfg(not(feature = "with-sql"))]
    pub async fn run<H: Hooks>(&self) -> eyre::Result<()> {
        let (environment, config, matches) = self.pre_run::<H>()?;
        let res = self
            .common_cli::<H>(&environment, &config, &matches)
            .await?;
        if res.is_some() {
            return Ok(());
        }

        // database specific commands
        let subcommand = matches.subcommand();
        let res = match subcommand {
            Some(("start", sub_matches)) => {
                commands::start::execute::<H>(sub_matches, &config, &environment).await
            }
            _ => {
                // Check if the user ran a custom command
                if let Some((name, sub_matches)) = subcommand {
                    if let Some(command) = self.custom_commands.get(name) {
                        command.execute(sub_matches, &config, &environment).await?;
                    } else {
                        eprintln!("Command not found: {}", name);
                        std::process::exit(101);
                    }
                } else {
                    eprintln!("Command not found");
                    std::process::exit(101);
                }

                Ok(())
            }
        };

        if let Err(e) = res {
            // utils::log_backtrace(&e);
            eprintln!("Error: {}", e);

            std::process::exit(101);
        }

        Ok(())
    }

    #[cfg(feature = "with-sql")]
    pub async fn run<H: Hooks, M: MigratorTrait>(&self) -> eyre::Result<()> {
        let (environment, config, matches) = self.pre_run::<H>()?;
        let res = self
            .common_cli::<H>(&environment, &config, &matches)
            .await?;
        if res.is_some() {
            return Ok(());
        }

        // database specific commands
        let subcommand = matches.subcommand();
        let res = match subcommand {
            Some(("start", sub_matches)) => {
                commands::start::execute::<H, M>(sub_matches, &config, &environment).await
            }
            Some(("sql", sub_matches)) => {
                commands::sql::execute::<H, M>(sub_matches, &config, &environment).await
            }
            _ => {
                // Check if the user ran a custom command
                if let Some((name, sub_matches)) = subcommand {
                    if let Some(command) = self.custom_commands.get(name) {
                        command.execute(sub_matches, &config, &environment).await?;
                    } else {
                        eprintln!("Command not found: {}", name);
                        std::process::exit(101);
                    }
                } else {
                    eprintln!("Command not found");
                    std::process::exit(101);
                }

                Ok(())
            }
        };

        if let Err(e) = res {
            // utils::log_backtrace(&e);
            eprintln!("Error: {}", e);

            std::process::exit(101);
        }

        Ok(())
    }
}
