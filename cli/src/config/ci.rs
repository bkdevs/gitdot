use serde::{Deserialize, Serialize};

use crate::executor::ExecutorType;

pub const SYSTEM_USER: &str = "gitdot";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CiConfig {
    pub runner_token: Option<String>,

    #[serde(default)]
    pub executor: ExecutorType,
}

impl Default for CiConfig {
    fn default() -> Self {
        Self {
            runner_token: None,
            executor: ExecutorType::default(),
        }
    }
}
