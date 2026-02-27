mod config;
mod create_build;
mod list_builds;

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub use config::CiConfig;
pub use create_build::CreateBuildRequest;
pub use list_builds::ListBuildsRequest;

use crate::model::{Build, BuildStatus, BuildTrigger};

#[derive(Debug, Clone)]
pub struct BuildResponse {
    pub id: Uuid,
    pub number: i32,
    pub repository_id: Uuid,
    pub trigger: BuildTrigger,
    pub commit_sha: String,
    pub status: BuildStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Build> for BuildResponse {
    fn from(build: Build) -> Self {
        Self {
            id: build.id,
            number: build.number,
            repository_id: build.repository_id,
            trigger: build.trigger,
            commit_sha: build.commit_sha,
            status: build.status,
            created_at: build.created_at,
            updated_at: build.updated_at,
        }
    }
}

pub type ListBuildsResponse = Vec<BuildResponse>;
