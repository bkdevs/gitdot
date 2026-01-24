mod create_repository;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::Repository;

pub use create_repository::CreateRepositoryRequest;

#[derive(Debug, Clone)]
pub struct RepositoryResponse {
    pub id: Uuid,
    pub name: String,
    pub owner: String,
    pub visibility: String,
    pub created_at: DateTime<Utc>,
}

impl From<Repository> for RepositoryResponse {
    fn from(repo: Repository) -> Self {
        Self {
            id: repo.id,
            name: repo.name,
            owner: repo.owner_name,
            visibility: repo.visibility.into(),
            created_at: repo.created_at,
        }
    }
}
