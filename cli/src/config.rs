#[cfg(feature = "main")]
pub mod user;
#[cfg(feature = "main")]
pub use user::UserConfig;

#[cfg(feature = "ci")]
pub mod runner;
#[cfg(feature = "ci")]
pub use runner::RunnerConfig;
