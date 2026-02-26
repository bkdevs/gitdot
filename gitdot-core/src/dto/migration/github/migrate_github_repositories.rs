use uuid::Uuid;

use crate::{
    dto::OwnerName,
    model::{MigrationRepository, RepositoryOwnerType},
};

#[derive(Debug, Clone)]
pub struct MigrateGitHubRepositoriesRequest {
    pub installation_id: i64,
    pub owner_id: Uuid,
    pub owner_name: OwnerName,
    pub owner_type: RepositoryOwnerType,
    pub migration_id: Uuid,
    pub migration_repositories: Vec<MigrationRepository>,
}

#[derive(Debug, Clone)]
pub struct MigrateGitHubRepositoriesResponse {
    pub migrated_repositories: Vec<MigratedRepositoryInfo>,
}

#[derive(Debug, Clone)]
pub struct MigratedRepositoryInfo {
    pub owner_name: String,
    pub repo_name: String,
    pub head_sha: Option<String>,
}
