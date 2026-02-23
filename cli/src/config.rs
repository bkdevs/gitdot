#[cfg(feature = "main")]
pub mod profile;

pub mod user;
pub use user::UserConfig;

#[cfg(feature = "ci")]
pub mod runner;
#[cfg(feature = "ci")]
pub use runner::RunnerConfig;
