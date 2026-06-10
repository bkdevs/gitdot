use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::ReviewError,
};

#[derive(Debug, Clone)]
pub struct RemoveReviewReviewerRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
    pub reviewer_name: OwnerName,
}

impl RemoveReviewReviewerRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        number: i32,
        reviewer_name: &str,
    ) -> Result<Self, ReviewError> {
        Ok(Self {
            owner: OwnerName::parse(owner, "owner name")?,
            repo: RepositoryName::parse(repo, "repository name")?,
            number,
            reviewer_name: OwnerName::parse(reviewer_name, "owner name")?,
        })
    }
}
