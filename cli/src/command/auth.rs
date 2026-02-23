mod login;
mod status;

use crate::config::Config;
use clap::{Args, Subcommand};

pub use login::login;
pub use status::get_status;

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
    pub async fn execute(&self, config: Config) -> anyhow::Result<()> {
        match self {
            AuthCommand::Login {} => login(config).await,
            AuthCommand::Status {} => get_status(config).await,
        }
    }
}
