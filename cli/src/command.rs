mod auth;

use clap::Parser;

use auth::{AuthArgs, AuthCommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub enum Args {
    /// Manage authentication credentials for gitdot account
    Auth(AuthArgs),
}

impl Args {
    pub async fn run(&self) -> anyhow::Result<()> {
        match &self {
            Args::Auth(args) => match &args.command {
                AuthCommand::Login => auth::login().await?,
                AuthCommand::Status => auth::get_status().await?,
            },
        }
        Ok(())
    }
}
