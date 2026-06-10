use uuid::Uuid;

use crate::{
    dto::{Cursor, DEFAULT_PER_PAGE_LIMIT, MAX_PER_PAGE_LIMIT, OwnerName, RepositoryName},
    error::{InputError, RepositoryError},
    util::cursor,
};

#[derive(Debug, Clone)]
pub struct ListRepositoryCommitFiltersRequest {
    pub user_id: Option<Uuid>,
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub cursor: Option<Cursor>,
    pub limit: u32,
}

impl ListRepositoryCommitFiltersRequest {
    pub fn new(
        user_id: Option<Uuid>,
        owner: &str,
        repo: &str,
        cursor: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Self, RepositoryError> {
        let owner = OwnerName::parse(owner, "owner name")?;
        let repo =
            RepositoryName::try_new(repo).map_err(|e| InputError::new("repository name", e))?;
        let cursor = cursor.map(cursor::decode).transpose()?;
        Ok(Self {
            user_id,
            owner,
            repo,
            cursor,
            limit: limit
                .unwrap_or(DEFAULT_PER_PAGE_LIMIT)
                .clamp(1, MAX_PER_PAGE_LIMIT),
        })
    }
}
