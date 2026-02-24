mod create_build;

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::Build;

pub use create_build::CreateBuildRequest;

#[derive(Debug, Clone)]
pub struct BuildResponse {
    pub id: Uuid,
    pub repo_owner: String,
    pub repo_name: String,
    pub task_dependencies: HashMap<Uuid, Vec<Uuid>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Build> for BuildResponse {
    fn from(build: Build) -> Self {
        Self {
            id: build.id,
            repo_owner: build.repo_owner,
            repo_name: build.repo_name,
            task_dependencies: build.task_dependencies.0,
            created_at: build.created_at,
            updated_at: build.updated_at,
        }
    }
}
