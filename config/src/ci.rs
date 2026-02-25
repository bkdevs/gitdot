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

    pub fn get_build_config(&self, trigger: &BuildTrigger) -> Result<&BuildConfig, CiConfigError> {
        self.builds
            .iter()
            .find(|b| &b.trigger == trigger)
            .ok_or_else(|| CiConfigError::NoMatchingBuild(String::from(trigger.clone())))
    }

    pub fn get_task_configs(&self, build: &BuildConfig) -> Vec<&TaskConfig> {
        self.tasks
            .iter()
            .filter(|t| build.tasks.contains(&t.name))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildTrigger {
    PullRequest,
    PushToMain,
}

impl From<BuildTrigger> for String {
    fn from(trigger: BuildTrigger) -> String {
        match trigger {
            BuildTrigger::PullRequest => "pull_request".to_string(),
            BuildTrigger::PushToMain => "push_to_main".to_string(),
        }
    }
}

impl TryFrom<String> for BuildTrigger {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "pull_request" => Ok(BuildTrigger::PullRequest),
            "push_to_main" => Ok(BuildTrigger::PushToMain),
            other => Err(format!("invalid trigger: {other}")),
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
    pub waits_for: Option<Vec<String>>,
}
