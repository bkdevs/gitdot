use uuid::Uuid;

use crate::error::{InputError, ReviewError};

use crate::dto::common::{OwnerName, RepositoryName};

#[derive(Debug, Clone)]
pub struct ResolveReviewCommentRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
    pub comment_id: Uuid,
    pub user_id: Uuid,
    pub resolved: bool,
}

impl ResolveReviewCommentRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        number: i32,
        comment_id: Uuid,
        user_id: Uuid,
        resolved: bool,
    ) -> Result<Self, ReviewError> {
        Ok(Self {
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            number,
            comment_id,
            user_id,
            resolved,
        })
    }
}
