use serde::{Deserialize, Serialize};

use super::error::CiConfigError;
use super::validate::validate_ci_config;

#[derive(Debug, Clone, Deserialize)]
pub struct CiConfig {
    pub builds: Vec<BuildConfig>,
    pub tasks: Vec<TaskConfig>,
}

impl CiConfig {
    pub fn new(toml: &str) -> Result<Self, CiConfigError> {
        let config: CiConfig = toml::from_str(toml).map_err(CiConfigError::Parse)?;
        validate_ci_config(&config)?;
        Ok(config)
    }

    pub fn get_build_config(&self, trigger: &BuildTrigger) -> Result<&BuildConfig, CiConfigError> {
        self.builds
            .iter()
            .find(|b| &b.trigger == trigger)
            .ok_or_else(|| CiConfigError::NoMatchingBuild(trigger.clone().into()))
    }

    pub fn get_task_configs(&self, build: &BuildConfig) -> Vec<&TaskConfig> {
        self.tasks
            .iter()
            .filter(|t| build.tasks.contains(&t.name))
            .collect()
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
    pub waits_for: Option<Vec<String>>,
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

impl TryFrom<&str> for BuildTrigger {
    type Error = CiConfigError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "pull_request" => Ok(BuildTrigger::PullRequest),
            "push_to_main" => Ok(BuildTrigger::PushToMain),
            other => Err(CiConfigError::InvalidTrigger(other.to_string())),
        }
    }
}
