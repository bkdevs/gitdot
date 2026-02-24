mod config;
mod create_build;

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub use config::{BuildConfig, BuildTrigger, CiConfig, CiConfigError, TaskConfig};
pub use create_build::CreateBuildRequest;

use crate::dto::TaskResponse;

#[derive(Debug, Clone)]
pub struct BuildResponse {
    pub id: Uuid,
    pub repo_owner: String,
    pub repo_name: String,
    pub trigger: String,
    pub commit_sha: String,
    pub task_dependencies: HashMap<Uuid, Vec<Uuid>>,
    pub tasks: Vec<TaskResponse>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
