pub mod docker;
pub mod local;
pub mod state;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::executor::state::ExecutorState;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExecutorType {
    Local,
    Docker,
}

impl ExecutorType {
    pub async fn execute(&self, state: ExecutorState) -> Result<()> {
        match self {
            ExecutorType::Local => local::execute(state).await,
            ExecutorType::Docker => docker::execute(state).await,
        }
    }
}
