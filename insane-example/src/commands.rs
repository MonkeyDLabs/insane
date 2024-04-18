use insane_cli::error;
use insane_cli::commands::*;
use insane_core::config::InsaneConfig;
use insane_core::environment::Environment;

pub struct TestUserCommand;

#[async_trait::async_trait]
impl CommandCustom for TestUserCommand {
    fn name(&self) -> &str {
        "test-user"
    }

    fn make_subcommand(&self) -> Command {
        Command::new("test-user").about("test about...").arg(
            Arg::new("test")
                .short('t')
                .long("test")
                .num_args(1)
                .default_value("testttt")
                // .value_parser(NonEmptyStringValueParser::new())
                .help("tesssss to listen on for HTTP connections"),
        )
    }

    async fn execute(&self, args: &ArgMatches, _config: &InsaneConfig, _env: &Environment) -> error::Result<()> {
        println!("TestUserCommand...{:?}", args);
        Ok(())
    }
}
