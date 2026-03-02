pub mod local;

use anyhow::Result;
use gitdot_api::resource::PollTaskResource;

use crate::config::RunnerConfig;

pub trait Executor: Sized {
    async fn initialize(config: &RunnerConfig, task: &PollTaskResource) -> Result<Self>;
    async fn execute(&self) -> Result<()>;
    async fn cleanup(self) -> Result<()>;
}
