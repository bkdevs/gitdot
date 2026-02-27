mod config;
mod create_build;
mod list_builds;

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub use config::CiConfig;
pub use create_build::CreateBuildRequest;
pub use list_builds::ListBuildsRequest;

use crate::model::{BuildStatus, BuildTrigger, BuildWithStats};

#[derive(Debug, Clone)]
pub struct BuildResponse {
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

impl From<BuildWithStats> for BuildResponse {
    fn from(build: BuildWithStats) -> Self {
        Self {
            id: build.id,
            number: build.number,
            repository_id: build.repository_id,
            trigger: build.trigger,
            commit_sha: build.commit_sha,
            status: build.status,
            total_tasks: build.total_tasks,
            completed_tasks: build.completed_tasks,
            created_at: build.created_at,
            updated_at: build.updated_at,
        }
    }
}

pub type ListBuildsResponse = Vec<BuildResponse>;
