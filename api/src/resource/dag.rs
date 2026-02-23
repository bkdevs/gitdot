use std::collections::HashMap;

use api_derive::ApiResource;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DagResource {
    pub id: Uuid,
    pub repo_owner: String,
    pub repo_name: String,
    pub task_dependencies: HashMap<Uuid, Vec<Uuid>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
