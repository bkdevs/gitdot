mod config;
mod create_build;
mod list_builds;

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub use config::{BuildConfig, BuildTrigger, CiConfig, CiConfigError, TaskConfig};
pub use create_build::CreateBuildRequest;
pub use list_builds::ListBuildsRequest;

use crate::{dto::TaskResponse, model::Build};

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

impl From<Build> for BuildResponse {
    fn from(build: Build) -> Self {
        Self {
            id: build.id,
            repo_owner: build.repo_owner,
            repo_name: build.repo_name,
            trigger: build.trigger,
            commit_sha: build.commit_sha,
            task_dependencies: build.task_dependencies.0,
            tasks: vec![],
            created_at: build.created_at,
            updated_at: build.updated_at,
        }
    }
}

pub type ListBuildsResponse = Vec<BuildResponse>;
