#[cfg(feature = "main")]
pub mod auth;
#[cfg(feature = "main")]
use auth::AuthArgs;

#[cfg(feature = "ci")]
pub mod ci;
#[cfg(feature = "ci")]
use ci::CiArgs;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub enum Args {
    #[cfg(feature = "main")]
    /// Manage authentication credentials for gitdot account
    Auth(AuthArgs),

    #[cfg(feature = "ci")]
    /// CI commands
    Ci(CiArgs),
}
