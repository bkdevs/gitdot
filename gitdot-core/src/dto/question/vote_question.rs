use uuid::Uuid;

use crate::{
    dto::{OwnerName, RepositoryName},
    error::{InputError, QuestionError},
};

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
            return Err(
                InputError::new("vote value", format!("{value}. Must be -1, 0, or 1")).into(),
            );
        }
        Ok(Self {
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
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
