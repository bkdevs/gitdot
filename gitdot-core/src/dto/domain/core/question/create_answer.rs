use uuid::Uuid;

use crate::{
    dto::{OwnerName, RepositoryName},
    error::QuestionError,
};

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
            owner: OwnerName::parse(owner, "owner name")?,
            repo: RepositoryName::parse(repo, "repository name")?,
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
