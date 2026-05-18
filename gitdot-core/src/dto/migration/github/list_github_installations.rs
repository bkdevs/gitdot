use uuid::Uuid;

use crate::{
    dto::common::{Cursor, DEFAULT_PER_PAGE_LIMIT, MAX_PER_PAGE_LIMIT},
    error::MigrationError,
    util::cursor,
};

#[derive(Debug, Clone)]
pub struct ListGitHubInstallationsRequest {
    pub owner_id: Uuid,
    pub cursor: Option<Cursor>,
    pub limit: u32,
}

impl ListGitHubInstallationsRequest {
    pub fn new(
        owner_id: Uuid,
        cursor: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Self, MigrationError> {
        let cursor = cursor.map(cursor::decode).transpose()?;
        Ok(Self {
            owner_id,
            cursor,
            limit: limit
                .unwrap_or(DEFAULT_PER_PAGE_LIMIT)
                .clamp(1, MAX_PER_PAGE_LIMIT),
        })
    }
}
