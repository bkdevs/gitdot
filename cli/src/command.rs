pub mod auth;

use clap::Parser;

use auth::AuthArgs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub enum Args {
    /// Manage authentication credentials for gitdot account
    Auth(AuthArgs),
}
