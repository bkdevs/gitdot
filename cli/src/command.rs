#[cfg(feature = "main")]
pub mod auth;
#[cfg(feature = "main")]
use auth::AuthArgs;

#[cfg(feature = "runner")]
pub mod runner;
#[cfg(feature = "runner")]
use runner::RunnerArgs;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub enum Args {
    #[cfg(feature = "main")]
    /// Manage authentication credentials for gitdot account
    Auth(AuthArgs),

    #[cfg(feature = "runner")]
    /// Install, configure, and manage the gitdot runner
    Runner(RunnerArgs),
}
