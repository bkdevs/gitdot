mod get_migration;
mod github;
mod list_migrations;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::{
    Migration, MigrationOrigin, MigrationRepository, MigrationRepositoryStatus, MigrationStatus,
};

pub use get_migration::GetMigrationRequest;
pub use github::{
    CreateGitHubInstallationRequest, CreateGitHubMigrationRequest, CreateGitHubMigrationResponse,
    GitHubInstallationResponse, GitHubRepositoryResponse,
    ListGitHubInstallationRepositoriesResponse, ListGitHubInstallationsRequest,
    ListGitHubInstallationsResponse, MigrateGitHubRepositoriesRequest,
    MigrateGitHubRepositoriesResponse, MigratedRepositoryInfo,
};
pub use list_migrations::{ListMigrationsRequest, ListMigrationsResponse};

#[derive(Debug, Clone)]
pub struct MigrationResponse {
    pub id: Uuid,
    pub number: i32,
    pub author_id: Uuid,
    pub origin: MigrationOrigin,
    pub status: MigrationStatus,
    pub repositories: Vec<MigrationRepositoryResponse>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl MigrationResponse {
    pub fn from_parts(migration: Migration, repositories: Vec<MigrationRepository>) -> Self {
        Self {
            id: migration.id,
            number: migration.number,
            author_id: migration.author_id,
            origin: migration.origin,
            status: migration.status,
            repositories: repositories.into_iter().map(Into::into).collect(),
            created_at: migration.created_at,
            updated_at: migration.updated_at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MigrationRepositoryResponse {
    pub id: Uuid,
    pub repository_id: Option<Uuid>,
    pub full_name: String,
    pub status: MigrationRepositoryStatus,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<MigrationRepository> for MigrationRepositoryResponse {
    fn from(r: MigrationRepository) -> Self {
        Self {
            id: r.id,
            repository_id: r.repository_id,
            full_name: r.full_name,
            status: r.status,
            error: r.error,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}
