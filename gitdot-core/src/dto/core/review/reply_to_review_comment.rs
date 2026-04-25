use uuid::Uuid;

use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::{InputError, ReviewError},
};

#[derive(Debug, Clone)]
pub struct ReplyToReviewCommentRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
    pub comment_id: Uuid,
    pub author_id: Uuid,
    pub body: String,
}

impl ReplyToReviewCommentRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        number: i32,
        comment_id: Uuid,
        author_id: Uuid,
        body: String,
    ) -> Result<Self, ReviewError> {
        Ok(Self {
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            number,
            comment_id,
            author_id,
            body,
        })
    }
}
