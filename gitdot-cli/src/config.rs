mod default;

#[cfg(feature = "main")]
pub mod user;
#[cfg(feature = "main")]
pub use user::UserConfig;

#[cfg(feature = "runner")]
pub mod runner;
#[cfg(feature = "runner")]
pub use runner::RunnerConfig;
