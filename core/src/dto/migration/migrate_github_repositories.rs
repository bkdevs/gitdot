use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    dto::OwnerName,
    error::MigrationError,
    model::{
        Migration, MigrationOrigin, MigrationRepository, MigrationRepositoryStatus,
        MigrationStatus, RepositoryOwnerType,
    },
};

#[derive(Debug, Clone)]
pub struct MigrateGitHubRepositoriesRequest {
    pub installation_id: i64,
    pub owner_name: OwnerName,
    pub owner_type: RepositoryOwnerType,
    pub repositories: Vec<String>,
    pub user_id: Uuid,
}

impl MigrateGitHubRepositoriesRequest {
    pub fn new(
        installation_id: i64,
        owner_name: &str,
        owner_type: &str,
        repositories: Vec<String>,
        user_id: Uuid,
    ) -> Result<Self, MigrationError> {
        Ok(Self {
            installation_id,
            owner_name: OwnerName::try_new(owner_name)
                .map_err(|e| MigrationError::OwnerNotFound(e.to_string()))?,
            owner_type: RepositoryOwnerType::try_from(owner_type)
                .map_err(|e| MigrationError::OwnerNotFound(e.to_string()))?,
            repositories,
            user_id,
        })
    }
}

#[derive(Debug, Clone)]
pub struct MigrationResponse {
    pub id: Uuid,
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
