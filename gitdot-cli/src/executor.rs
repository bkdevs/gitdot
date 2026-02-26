pub mod docker;
pub mod local;

use anyhow::Result;
use gitdot_api::resource::PollTaskResource;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
pub trait Executor {
    const TYPE: ExecutorType;

    async fn execute(&self, task: &PollTaskResource) -> Result<()>;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExecutorType {
    #[default]
    Local,
    Docker,
}
