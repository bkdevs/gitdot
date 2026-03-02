use std::path::PathBuf;

use anyhow::Result;
use gitdot_api::resource::PollTaskResource;
use s2_sdk::S2;

use crate::config::RunnerConfig;

pub struct ExecutorState {
    pub working_directory: PathBuf,
    pub task: PollTaskResource,
    pub s2: S2,
}

impl ExecutorState {
    pub async fn initialize(config: &RunnerConfig, task: PollTaskResource) -> Result<Self> {
        let working_directory = PathBuf::from(format!("/tmp/gitdot/tasks/{}", task.id));
        tokio::fs::create_dir_all(&working_directory).await?;

        let s2 = S2::from_url(&config.s2_server_url)?;
        Ok(Self {
            working_directory,
            task,
            s2,
        })
    }
}
