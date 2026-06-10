use crate::{
    dto::common::{Cursor, DEFAULT_PER_PAGE_LIMIT, MAX_PER_PAGE_LIMIT, OwnerName, RepositoryName},
    error::WebhookError,
    util::cursor,
};

#[derive(Debug, Clone)]
pub struct ListWebhooksRequest {
    pub owner_name: OwnerName,
    pub repo_name: RepositoryName,
    pub cursor: Option<Cursor>,
    pub limit: u32,
}

impl ListWebhooksRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        cursor: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Self, WebhookError> {
        let owner_name = OwnerName::parse(owner, "owner name")?;
        let repo_name = RepositoryName::parse(repo, "repository name")?;
        let cursor = cursor.map(cursor::decode).transpose()?;
        Ok(Self {
            owner_name,
            repo_name,
            cursor,
            limit: limit
                .unwrap_or(DEFAULT_PER_PAGE_LIMIT)
                .clamp(1, MAX_PER_PAGE_LIMIT),
        })
    }
}
