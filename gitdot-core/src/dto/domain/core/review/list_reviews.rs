use uuid::Uuid;

use crate::{
    dto::common::{Cursor, DEFAULT_PER_PAGE_LIMIT, MAX_PER_PAGE_LIMIT, OwnerName, RepositoryName},
    error::{InputError, ReviewError},
    util::cursor,
};

#[derive(Debug, Clone)]
pub struct ListReviewsRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub viewer_id: Option<Uuid>,
    pub cursor: Option<Cursor>,
    pub limit: u32,
}

impl ListReviewsRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        viewer_id: Option<Uuid>,
        cursor: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Self, ReviewError> {
        let owner = OwnerName::parse(owner, "owner name")?;
        let repo =
            RepositoryName::try_new(repo).map_err(|e| InputError::new("repository name", e))?;
        let cursor = cursor.map(cursor::decode).transpose()?;
        Ok(Self {
            owner,
            repo,
            viewer_id,
            cursor,
            limit: limit
                .unwrap_or(DEFAULT_PER_PAGE_LIMIT)
                .clamp(1, MAX_PER_PAGE_LIMIT),
        })
    }
}
