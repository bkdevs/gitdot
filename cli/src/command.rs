mod auth;
mod credential_helper;

use clap::Parser;

use auth::{AuthArgs, AuthCommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub enum Args {
    /// Manage authentication credentials for gitdot account
    Auth(AuthArgs),

    /// Git credential helper (used internally by git)
    #[command(name = "credential-helper")]
    CredentialHelper,
}

impl Args {
    pub async fn run(&self) -> anyhow::Result<()> {
        match &self {
            Args::Auth(args) => match &args.command {
                AuthCommand::Login => auth::login().await?,
                AuthCommand::Logout => auth::logout().await?,
                AuthCommand::Status => auth::get_status().await?,
            },
            Args::CredentialHelper => credential_helper::run()?,
        }
        Ok(())
    }
}
