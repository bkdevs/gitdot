use uuid::Uuid;

use crate::{
    dto::OwnerName,
    error::MigrationError,
    model::{Migration, MigrationRepository, RepositoryOwnerType},
};

#[derive(Debug, Clone)]
pub struct CreateGitHubMigrationRequest {
    pub owner_name: OwnerName,
    pub owner_type: RepositoryOwnerType,
    pub repositories: Vec<String>,
    pub user_id: Uuid,
}

impl CreateGitHubMigrationRequest {
    pub fn new(
        owner_name: &str,
        owner_type: &str,
        repositories: Vec<String>,
        user_id: Uuid,
    ) -> Result<Self, MigrationError> {
        Ok(Self {
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
pub struct CreateGitHubMigrationResponse {
    pub migration: Migration,
    pub migration_repositories: Vec<MigrationRepository>,
    pub owner_id: Uuid,
    pub owner_name: OwnerName,
    pub owner_type: RepositoryOwnerType,
}
