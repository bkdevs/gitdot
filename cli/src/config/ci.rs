use serde::{Deserialize, Serialize};

use crate::executor::ExecutorType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CiConfig {
    pub runner_token: Option<String>,

    #[serde(default = "default_run_as_user")]
    pub run_as_user: String,

    #[serde(default)]
    pub executor: ExecutorType,
}

impl Default for CiConfig {
    fn default() -> Self {
        Self {
            runner_token: None,
            run_as_user: default_run_as_user(),
            executor: ExecutorType::default(),
        }
    }
}

fn default_run_as_user() -> String {
    "gitdot-runner".to_string()
}
