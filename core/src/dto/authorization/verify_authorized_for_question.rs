use uuid::Uuid;

use crate::dto::{OwnerName, RepositoryName};
use crate::error::AuthorizationError;

#[derive(Debug, Clone)]
pub struct QuestionAuthorizationRequest {
    pub user_id: Uuid,
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
}

impl QuestionAuthorizationRequest {
    pub fn new(
        user_id: Uuid,
        owner_name: &str,
        repo_name: &str,
        number: i32,
    ) -> Result<Self, AuthorizationError> {
        Ok(Self {
            user_id: user_id,
            owner: OwnerName::try_new(owner_name)
                .map_err(|e| AuthorizationError::InvalidRequest(e.to_string()))?,
            repo: RepositoryName::try_new(repo_name)
                .map_err(|e| AuthorizationError::InvalidRequest(e.to_string()))?,
            number,
        })
    }

    pub fn get_repo_path(&self) -> String {
        format!("{}/{}", self.owner.as_ref(), self.repo.as_ref())
    }

    pub fn get_question_path(&self) -> String {
        format!(
            "{}/{}/{}",
            self.owner.as_ref(),
            self.repo.as_ref(),
            self.number
        )
    }
}
