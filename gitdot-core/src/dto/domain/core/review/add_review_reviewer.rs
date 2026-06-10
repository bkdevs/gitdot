use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::ReviewError,
};

#[derive(Debug, Clone)]
pub struct AddReviewReviewerReqeuest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
    pub user_name: OwnerName,
}

impl AddReviewReviewerReqeuest {
    pub fn new(owner: &str, repo: &str, number: i32, user_name: &str) -> Result<Self, ReviewError> {
        Ok(Self {
            owner: OwnerName::parse(owner, "owner name")?,
            repo: RepositoryName::parse(repo, "repository name")?,
            number,
            user_name: OwnerName::parse(user_name, "owner name")?,
        })
    }
}
