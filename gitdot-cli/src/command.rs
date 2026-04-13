#[cfg(feature = "main")]
mod add;
#[cfg(feature = "main")]
mod auth;
#[cfg(feature = "main")]
mod ci;
#[cfg(feature = "main")]
mod review;
#[cfg(feature = "runner")]
mod runner;

use clap::Parser;

#[cfg(feature = "main")]
use {add::AddArgs, auth::AuthArgs, ci::CiArgs, review::ReviewArgs};

#[cfg(feature = "runner")]
use runner::RunnerArgs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub enum Args {
    #[cfg(feature = "main")]
    /// Stage all changes and commit
    Add(AddArgs),

    #[cfg(feature = "main")]
    /// Manage authentication credentials
    Auth(AuthArgs),

    /// Create and manage code reviews
    #[cfg(feature = "main")]
    #[command(alias = "r")]
    Review(ReviewArgs),

    #[cfg(feature = "main")]
    /// Lint, format, and test CI configuration
    Ci(CiArgs),

    #[cfg(feature = "runner")]
    /// Install, configure, and manage the gitdot runner
    Runner(RunnerArgs),
}
