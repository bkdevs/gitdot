use uuid::Uuid;

use crate::dto::{OwnerName, RepositoryName};
use crate::error::QuestionError;

#[derive(Debug, Clone)]
pub struct VoteQuestionRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
    pub user_id: Uuid,
    pub value: i16,
}

impl VoteQuestionRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        number: i32,
        user_id: Uuid,
        value: i16,
    ) -> Result<Self, QuestionError> {
        if !(-1..=1).contains(&value) {
            return Err(QuestionError::InvalidVoteValue(value));
        }
        Ok(Self {
            owner: OwnerName::try_new(owner)
                .map_err(|e| QuestionError::InvalidOwnerName(e.to_string()))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| QuestionError::InvalidRepositoryName(e.to_string()))?,
            number,
            user_id,
            value,
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
