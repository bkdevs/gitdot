use uuid::Uuid;

use crate::{
    dto::OwnerName,
    error::MigrationError,
    model::{Migration, RepositoryOwnerType},
};

#[derive(Debug, Clone)]
pub struct CreateGitHubMigrationRequest {
    pub author_id: Uuid,
    pub installation_id: i64,
    pub origin: String,
    pub origin_type: RepositoryOwnerType,
    pub destination: OwnerName,
    pub destination_type: RepositoryOwnerType,
    pub repositories: Vec<String>,
}

impl CreateGitHubMigrationRequest {
    pub fn new(
        author_id: Uuid,
        installation_id: i64,
        origin: &str,
        origin_type: &str,
        destination: &str,
        destination_type: &str,
        repositories: Vec<String>,
    ) -> Result<Self, MigrationError> {
        Ok(Self {
            author_id,
            installation_id,
            origin: origin.to_string(),
            origin_type: RepositoryOwnerType::try_from(origin_type)
                .map_err(|e| MigrationError::OwnerNotFound(e.to_string()))?,
            destination: OwnerName::try_new(destination)
                .map_err(|e| MigrationError::OwnerNotFound(e.to_string()))?,
            destination_type: RepositoryOwnerType::try_from(destination_type)
                .map_err(|e| MigrationError::OwnerNotFound(e.to_string()))?,
            repositories,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CreateGitHubMigrationResponse {
    pub migration: Migration,
    pub owner_id: Uuid,
    pub owner_name: OwnerName,
    pub owner_type: RepositoryOwnerType,
}
