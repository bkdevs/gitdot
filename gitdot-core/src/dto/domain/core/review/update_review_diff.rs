use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::ReviewError,
};

#[derive(Debug, Clone)]
pub struct UpdateReviewDiffRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
    pub position: i32,
    pub message: Option<String>,
}

impl UpdateReviewDiffRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        number: i32,
        position: i32,
        message: Option<String>,
    ) -> Result<Self, ReviewError> {
        Ok(Self {
            owner: OwnerName::parse(owner, "owner name")?,
            repo: RepositoryName::parse(repo, "repository name")?,
            number,
            position,
            message,
        })
    }
}
