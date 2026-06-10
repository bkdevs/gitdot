use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::ReviewError,
};

#[derive(Debug, Clone)]
pub struct PublishReviewRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
}

impl PublishReviewRequest {
    pub fn new(owner: &str, repo: &str, number: i32) -> Result<Self, ReviewError> {
        Ok(Self {
            owner: OwnerName::parse(owner, "owner name")?,
            repo: RepositoryName::parse(repo, "repository name")?,
            number,
        })
    }
}
