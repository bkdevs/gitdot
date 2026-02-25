use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    pub number: i32,
    pub author_id: Uuid,

    pub origin_service: String,
    pub origin: String,
    pub origin_type: String,
    pub destination: String,
    pub destination_type: String,

    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub repositories: Vec<MigrationRepositoryResource>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationRepositoryResource {
    pub id: Uuid,

    pub origin_full_name: String,
    pub destination_full_name: String,

    pub status: String,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
