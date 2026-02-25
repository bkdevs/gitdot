mod get_migration;
mod github;
mod list_migrations;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::{
    Migration, MigrationOriginService, MigrationRepository, MigrationRepositoryStatus,
    MigrationStatus, RepositoryOwnerType,
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

    pub origin_service: MigrationOriginService,
    pub origin: String,
    pub origin_type: RepositoryOwnerType,
    pub destination: String,
    pub destination_type: RepositoryOwnerType,

    pub status: MigrationStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub repositories: Vec<MigrationRepositoryResponse>,
}

impl MigrationResponse {
    pub fn from_parts(migration: Migration, repositories: Vec<MigrationRepository>) -> Self {
        Self {
            id: migration.id,
            number: migration.number,
            author_id: migration.author_id,
            origin_service: migration.origin_service,
            origin: migration.origin,
            origin_type: migration.origin_type,
            destination: migration.destination,
            destination_type: migration.destination_type,
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

    pub origin_full_name: String,
    pub destination_full_name: String,

    pub status: MigrationRepositoryStatus,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<MigrationRepository> for MigrationRepositoryResponse {
    fn from(r: MigrationRepository) -> Self {
        Self {
            id: r.id,
            origin_full_name: r.origin_full_name,
            destination_full_name: r.destination_full_name,
            status: r.status,
            error: r.error,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}
