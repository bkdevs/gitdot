use chrono::{DateTime, Utc};

use crate::{
    dto::{Cursor, DEFAULT_PER_PAGE_LIMIT, MAX_PER_PAGE_LIMIT, OwnerName, RepositoryName},
    error::{CommitError, InputError},
    util::cursor,
};

#[derive(Debug, Clone)]
pub struct ListRepositoryCommitsRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub ref_name: String,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    pub cursor: Option<Cursor>,
    pub limit: u32,
}

impl ListRepositoryCommitsRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        ref_name: String,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
        cursor: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Self, CommitError> {
        if to.is_some() && from.is_none() {
            return Err(InputError::new("date range", "`to` requires `from` to be set").into());
        }
        let owner = OwnerName::parse(owner, "owner name")?;
        let repo =
            RepositoryName::try_new(repo).map_err(|e| InputError::new("repository name", e))?;
        let cursor = cursor.map(cursor::decode).transpose()?;
        Ok(Self {
            owner,
            repo,
            ref_name,
            from: from.unwrap_or(DateTime::<Utc>::UNIX_EPOCH),
            to: to.unwrap_or_else(Utc::now),
            cursor,
            limit: limit
                .unwrap_or(DEFAULT_PER_PAGE_LIMIT)
                .clamp(1, MAX_PER_PAGE_LIMIT),
        })
    }
}
