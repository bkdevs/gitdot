pub mod auth;
pub mod ci;

use clap::Parser;

use auth::AuthArgs;
use ci::CiArgs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub enum Args {
    /// Manage authentication credentials for gitdot account
    Auth(AuthArgs),

    /// CI commands
    Ci(CiArgs),
}
