mod auth;

use clap::{Parser, Subcommand};

use auth::{AuthArgs, AuthCommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Command,
}

impl Args {
    pub async fn run(&self) -> anyhow::Result<()> {
        match &self.command {
            Command::Auth(args) => match &args.command {
                AuthCommand::Login => auth::login().await?,
                AuthCommand::Status => auth::get_status().await?,
            },
        }
        Ok(())
    }
}

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    /// Manage authentication credentials for gitdot account
    Auth(AuthArgs),
}
