use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use api_derive::ApiResource;

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHubInstallationResource {
    pub id: Uuid,
    pub installation_id: i64,
    pub owner_id: Uuid,
    pub installation_type: String,
    pub github_login: String,
    pub created_at: DateTime<Utc>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHubRepositoryResource {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub private: bool,
    pub default_branch: String,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationResource {
    pub id: Uuid,
    pub author_id: Uuid,
    pub origin: String,
    pub status: String,
    pub repositories: Vec<MigrationRepositoryResource>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationRepositoryResource {
    pub id: Uuid,
    pub repository_id: Option<Uuid>,
    pub full_name: String,
    pub status: String,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
