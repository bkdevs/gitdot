use crate::dto::{OwnerName, RepositoryName};
use crate::error::QuestionError;

#[derive(Debug, Clone)]
pub struct GetQuestionsRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
}

impl GetQuestionsRequest {
    pub fn new(owner: &str, repo: &str) -> Result<Self, QuestionError> {
        Ok(Self {
            owner: OwnerName::try_new(owner)
                .map_err(|e| QuestionError::InvalidOwnerName(e.to_string()))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| QuestionError::InvalidOwnerName(e.to_string()))?,
        })
    }

    pub fn get_repo_path(&self) -> String {
        format!("{}/{}", self.owner.as_ref(), self.repo.as_ref())
    }
}
