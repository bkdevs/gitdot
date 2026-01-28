mod login;
mod status;

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
