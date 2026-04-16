use crate::dto::common::{OwnerName, RepositoryName};
use crate::error::{InputError, ReviewError};

use super::ReviewId;

#[derive(Debug, Clone)]
pub struct GetReviewRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub review_id: ReviewId,
}

impl GetReviewRequest {
    pub fn new(owner: &str, repo: &str, review_id: ReviewId) -> Result<Self, ReviewError> {
        Ok(Self {
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            review_id,
        })
    }

    pub fn get_review_path(&self) -> String {
        let id = match &self.review_id {
            ReviewId::Number(n) => n.to_string(),
            ReviewId::Hex(s) => s.clone(),
        };
        format!("{}/{}/review/{}", self.owner.as_ref(), self.repo.as_ref(), id)
    }
}
