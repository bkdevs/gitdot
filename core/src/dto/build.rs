mod config;
mod create_build;
mod list_builds;

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub use config::{BuildConfig, BuildTrigger, CiConfig, TaskConfig};
pub use create_build::CreateBuildRequest;
pub use list_builds::ListBuildsRequest;

use crate::model::Build;

#[derive(Debug, Clone)]
pub struct BuildResponse {
    pub id: Uuid,
    pub number: i32,
    pub repo_owner: String,
    pub repo_name: String,
    pub trigger: String,
    pub commit_sha: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Build> for BuildResponse {
    fn from(build: Build) -> Self {
        Self {
            id: build.id,
            number: build.number,
            repo_owner: build.repo_owner,
            repo_name: build.repo_name,
            trigger: build.trigger,
            commit_sha: build.commit_sha,
            created_at: build.created_at,
            updated_at: build.updated_at,
        }
    }
}

pub type ListBuildsResponse = Vec<BuildResponse>;
