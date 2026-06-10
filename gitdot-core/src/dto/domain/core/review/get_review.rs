use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::ReviewError,
};

#[derive(Debug, Clone)]
pub struct GetReviewRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
}

impl GetReviewRequest {
    pub fn new(owner: &str, repo: &str, number: i32) -> Result<Self, ReviewError> {
        Ok(Self {
            owner: OwnerName::parse(owner, "owner name")?,
            repo: RepositoryName::parse(repo, "repository name")?,
            number,
        })
    }

    pub fn get_review_path(&self) -> String {
        format!(
            "{}/{}/review/{}",
            self.owner.as_ref(),
            self.repo.as_ref(),
            self.number
        )
    }
}
