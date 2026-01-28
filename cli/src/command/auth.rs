mod login;
mod status;

use clap::{Parser, Subcommand};

pub(crate) use login::login;
pub(crate) use status::get_status;

#[derive(Parser, Debug)]
pub(crate) struct AuthArgs {
    #[command(subcommand)]
    pub command: AuthCommand,
}

#[derive(Subcommand, Debug)]
pub(crate) enum AuthCommand {
    /// Authenticate with gitdot OAuth and store your credentials locally
    Login,

    /// Display the current authentication state and logged-in user
    Status,
}
