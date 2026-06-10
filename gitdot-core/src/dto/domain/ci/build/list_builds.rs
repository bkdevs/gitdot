use crate::{
    dto::common::{Cursor, DEFAULT_PER_PAGE_LIMIT, MAX_PER_PAGE_LIMIT, OwnerName, RepositoryName},
    error::{BuildError, InputError},
    util::cursor,
};

#[derive(Debug, Clone)]
pub struct ListBuildsRequest {
    pub repo_owner: OwnerName,
    pub repo_name: RepositoryName,
    pub cursor: Option<Cursor>,
    pub limit: u32,
}

impl ListBuildsRequest {
    pub fn new(
        repo_owner: &str,
        repo_name: &str,
        cursor: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Self, BuildError> {
        let repo_owner = OwnerName::parse(repo_owner, "owner name")?;
        let repo_name = RepositoryName::try_new(repo_name)
            .map_err(|e| InputError::new("repository name", e))?;
        let cursor = cursor.map(cursor::decode).transpose()?;
        Ok(Self {
            repo_owner,
            repo_name,
            cursor,
            limit: limit
                .unwrap_or(DEFAULT_PER_PAGE_LIMIT)
                .clamp(1, MAX_PER_PAGE_LIMIT),
        })
    }
}
