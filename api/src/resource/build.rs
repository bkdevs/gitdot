use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use api_derive::ApiResource;

use crate::resource::TaskResource;

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildResource {
    pub id: Uuid,
    pub repo_owner: String,
    pub repo_name: String,
    pub trigger: String,
    pub commit_sha: String,
    pub build_config: String,
    pub tasks: Vec<TaskResource>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
