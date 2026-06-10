use uuid::Uuid;

use crate::{
    dto::{Cursor, DEFAULT_PER_PAGE_LIMIT, MAX_PER_PAGE_LIMIT, OwnerName, RepositoryName},
    error::QuestionError,
    util::cursor,
};

#[derive(Debug, Clone)]
pub struct ListQuestionsRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub user_id: Option<Uuid>,
    pub cursor: Option<Cursor>,
    pub limit: u32,
}

impl ListQuestionsRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        user_id: Option<Uuid>,
        cursor: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Self, QuestionError> {
        let owner = OwnerName::parse(owner, "owner name")?;
        let repo = RepositoryName::parse(repo, "repository name")?;
        let cursor = cursor.map(cursor::decode).transpose()?;
        Ok(Self {
            owner,
            repo,
            user_id,
            cursor,
            limit: limit
                .unwrap_or(DEFAULT_PER_PAGE_LIMIT)
                .clamp(1, MAX_PER_PAGE_LIMIT),
        })
    }

    pub fn get_repo_path(&self) -> String {
        format!("{}/{}", self.owner.as_ref(), self.repo.as_ref())
    }
}
