use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    dto::{OwnerName, RepositoryName},
    error::{InputError, QuestionError},
};

#[derive(Debug, Clone)]
pub struct ListQuestionsRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub user_id: Option<Uuid>,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}

impl ListQuestionsRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        user_id: Option<Uuid>,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Self, QuestionError> {
        Ok(Self {
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo).map_err(|e| InputError::new("owner name", e))?,
            user_id,
            from,
            to,
        })
    }

    pub fn get_repo_path(&self) -> String {
        format!("{}/{}", self.owner.as_ref(), self.repo.as_ref())
    }
}
