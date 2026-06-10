use uuid::Uuid;

use crate::{
    dto::{OwnerName, RepositoryName},
    error::{InputError, MigrationError},
    model::{Migration, RepositoryOwnerType},
};

#[derive(Debug, Clone)]
pub struct CreateGitHubMigrationRequest {
    pub user_id: Uuid,
    pub installation_id: i64,
    pub origin: String,
    pub origin_type: RepositoryOwnerType,
    pub destination: OwnerName,
    pub destination_type: RepositoryOwnerType,
    pub repositories: Vec<GitHubMigrationRepository>,
}

#[derive(Debug, Clone)]
pub struct GitHubMigrationRepository {
    pub name: RepositoryName,
    pub id: i64,
}

impl CreateGitHubMigrationRequest {
    pub fn new(
        user_id: Uuid,
        installation_id: i64,
        origin: &str,
        origin_type: &str,
        destination: &str,
        destination_type: &str,
        repositories: Vec<(String, i64)>,
    ) -> Result<Self, MigrationError> {
        let repositories = repositories
            .into_iter()
            .map(|(name, id)| {
                let name = RepositoryName::parse(&name, "repository name")?;
                Ok(GitHubMigrationRepository { name, id })
            })
            .collect::<Result<Vec<_>, MigrationError>>()?;
        Ok(Self {
            user_id,
            installation_id,
            origin: origin.to_string(),
            origin_type: RepositoryOwnerType::try_from(origin_type)
                .map_err(|e| InputError::new("origin type", e))?,
            destination: OwnerName::parse(destination, "destination")?,
            destination_type: RepositoryOwnerType::try_from(destination_type)
                .map_err(|e| InputError::new("destination type", e))?,
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
