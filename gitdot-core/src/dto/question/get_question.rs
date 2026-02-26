use uuid::Uuid;

use crate::{
    dto::{OwnerName, RepositoryName},
    error::QuestionError,
};

#[derive(Debug, Clone)]
pub struct GetQuestionRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
    pub user_id: Option<Uuid>,
}

impl GetQuestionRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        number: i32,
        user_id: Option<Uuid>,
    ) -> Result<Self, QuestionError> {
        Ok(Self {
            owner: OwnerName::try_new(owner)
                .map_err(|e| QuestionError::InvalidOwnerName(e.to_string()))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| QuestionError::InvalidOwnerName(e.to_string()))?,
            number,
            user_id,
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
