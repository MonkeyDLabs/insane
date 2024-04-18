use insane_cli::InsaneCli;
use insane_example::{commands::TestUserCommand, hook::App};
use migration::Migrator;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let mut cli = InsaneCli::new();
    cli.add_custom_command(TestUserCommand {});
    cli.run::<App, Migrator>().await
}
