mod login;
mod status;

use clap::{Args, Subcommand};

use crate::config::UserConfig;

#[derive(Args, Debug)]
pub struct AuthArgs {
    #[command(subcommand)]
    pub command: AuthCommand,
}

#[derive(Subcommand, Debug)]
pub enum AuthCommand {
    /// Authenticate with gitdot OAuth and store your credentials locally
    Login,

    /// Display the current authentication state and logged-in user
    Status,
}

impl AuthCommand {
    pub async fn execute(&self, config: UserConfig) -> anyhow::Result<()> {
        match self {
            AuthCommand::Login {} => login::login(config).await,
            AuthCommand::Status {} => status::get_status(config).await,
        }
    }
}
