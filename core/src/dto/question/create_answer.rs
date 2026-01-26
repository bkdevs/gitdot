use uuid::Uuid;

use crate::dto::{OwnerName, RepositoryName};
use crate::error::QuestionError;

#[derive(Debug, Clone)]
pub struct CreateAnswerRequest {
    pub author_id: Uuid,
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
    pub body: String,
}

impl CreateAnswerRequest {
    pub fn new(
        author_id: Uuid,
        owner: &str,
        repo: &str,
        number: i32,
        body: String,
    ) -> Result<Self, QuestionError> {
        Ok(Self {
            author_id,
            owner: OwnerName::try_new(owner)
                .map_err(|e| QuestionError::InvalidOwnerName(e.to_string()))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| QuestionError::InvalidOwnerName(e.to_string()))?,
            number,
            body,
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
