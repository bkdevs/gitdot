use serde::{Deserialize, Serialize};

use crate::validate::ci::{self, CiConfigError};

#[derive(Debug, Clone, Deserialize)]
pub struct CiConfig {
    pub builds: Vec<BuildConfig>,
    pub tasks: Vec<TaskConfig>,
}

impl CiConfig {
    pub fn new(toml: &str) -> Result<Self, CiConfigError> {
        let config: CiConfig = toml::from_str(toml).map_err(CiConfigError::Parse)?;
        ci::validate(&config)?;
        Ok(config)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildTrigger {
    PullRequest,
    PushToMain,
}

impl Into<String> for BuildTrigger {
    fn into(self) -> String {
        match self {
            BuildTrigger::PullRequest => "pull_request".to_string(),
            BuildTrigger::PushToMain => "push_to_main".to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct BuildConfig {
    pub trigger: BuildTrigger,
    pub tasks: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TaskConfig {
    pub name: String,
    pub command: String,
    pub runs_after: Option<Vec<String>>,
}
