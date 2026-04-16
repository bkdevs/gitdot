use uuid::Uuid;

use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::{InputError, ReviewError},
};

use super::ReviewId;

#[derive(Debug, Clone)]
pub struct UpdateReviewCommentRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub review_id: ReviewId,
    pub comment_id: Uuid,
    pub user_id: Uuid,
    pub body: String,
}

impl UpdateReviewCommentRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        review_id: ReviewId,
        comment_id: Uuid,
        user_id: Uuid,
        body: String,
    ) -> Result<Self, ReviewError> {
        Ok(Self {
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            review_id,
            comment_id,
            user_id,
            body,
        })
    }
}
