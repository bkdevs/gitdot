use chrono::{DateTime, Utc};
use sqlx::{FromRow, Type};
use uuid::Uuid;

use crate::error::BuildError;

#[derive(Debug, Clone, FromRow)]
pub struct Build {
    pub id: Uuid,
    pub number: i32,
    pub repository_id: Uuid,
    pub trigger: BuildTrigger,
    pub commit_sha: String,
    pub status: BuildStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct BuildWithStats {
    pub id: Uuid,
    pub number: i32,
    pub repository_id: Uuid,
    pub trigger: BuildTrigger,
    pub commit_sha: String,
    pub status: BuildStatus,
    pub total_tasks: i32,
    pub completed_tasks: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "build_trigger", rename_all = "snake_case")]
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

impl From<gitdot_config::ci::BuildTrigger> for BuildTrigger {
    fn from(trigger: gitdot_config::ci::BuildTrigger) -> Self {
        match trigger {
            gitdot_config::ci::BuildTrigger::PullRequest => BuildTrigger::PullRequest,
            gitdot_config::ci::BuildTrigger::PushToMain => BuildTrigger::PushToMain,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "build_status", rename_all = "lowercase")]
pub enum BuildStatus {
    Running,
    Success,
    Failure,
}

impl TryFrom<&str> for BuildStatus {
    type Error = BuildError;

    fn try_from(status: &str) -> Result<Self, Self::Error> {
        match status {
            "running" => Ok(BuildStatus::Running),
            "success" => Ok(BuildStatus::Success),
            "failure" => Ok(BuildStatus::Failure),
            _ => Err(BuildError::InvalidStatus(status.to_string())),
        }
    }
}

impl Into<String> for BuildStatus {
    fn into(self) -> String {
        match self {
            BuildStatus::Running => "running".to_string(),
            BuildStatus::Success => "success".to_string(),
            BuildStatus::Failure => "failure".to_string(),
        }
    }
}
