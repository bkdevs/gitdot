use uuid::Uuid;

use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::ReviewError,
};

#[derive(Debug, Clone)]
pub struct ReplyToReviewCommentRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
    pub comment_id: Uuid,
    pub user_id: Uuid,
    pub body: String,
}

impl ReplyToReviewCommentRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        number: i32,
        comment_id: Uuid,
        user_id: Uuid,
        body: String,
    ) -> Result<Self, ReviewError> {
        Ok(Self {
            owner: OwnerName::parse(owner, "owner name")?,
            repo: RepositoryName::parse(repo, "repository name")?,
            number,
            comment_id,
            user_id,
            body,
        })
    }
}
