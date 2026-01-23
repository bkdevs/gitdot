mod create_repository;

use chrono::{DateTime, Utc};
use nutype::nutype;
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

#[nutype(
    sanitize(trim, lowercase),
    validate(not_empty, len_char_max = 100),
    derive(
        Debug,
        Clone,
        Serialize,
        Deserialize,
        PartialEq,
        Eq,
        TryFrom,
        AsRef,
        Deref,
    )
)]
pub struct RepositoryName(String);

#[nutype(
    sanitize(trim, lowercase),
    validate(not_empty, len_char_max = 100),
    derive(
        Debug,
        Clone,
        Serialize,
        Deserialize,
        PartialEq,
        Eq,
        TryFrom,
        AsRef,
        Deref,
    )
)]
pub struct OwnerName(String);
