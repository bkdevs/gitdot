use crate::{
    dto::{OwnerName, RepositoryName},
    error::QuestionError,
};

#[derive(Debug, Clone)]
pub struct UpdateQuestionRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
    pub title: String,
    pub body: String,
}

impl UpdateQuestionRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        number: i32,
        title: String,
        body: String,
    ) -> Result<Self, QuestionError> {
        Ok(Self {
            owner: OwnerName::try_new(owner)
                .map_err(|e| QuestionError::InvalidOwnerName(e.to_string()))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| QuestionError::InvalidOwnerName(e.to_string()))?,
            number,
            title,
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
