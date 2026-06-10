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
            owner: OwnerName::parse(owner, "owner name")?,
            repo: RepositoryName::parse(repo, "repository name")?,
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
