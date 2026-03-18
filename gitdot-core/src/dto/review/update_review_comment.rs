use uuid::Uuid;

use crate::error::ReviewError;

use super::super::common::{OwnerName, RepositoryName};

#[derive(Debug, Clone)]
pub struct UpdateReviewCommentRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
    pub comment_id: Uuid,
    pub user_id: Uuid,
    pub body: String,
}

impl UpdateReviewCommentRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        number: i32,
        comment_id: Uuid,
        user_id: Uuid,
        body: String,
    ) -> Result<Self, ReviewError> {
        Ok(Self {
            owner: OwnerName::try_new(owner)
                .map_err(|e| ReviewError::InvalidOwnerName(e.to_string()))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| ReviewError::InvalidRepositoryName(e.to_string()))?,
            number,
            comment_id,
            user_id,
            body,
        })
    }
}
