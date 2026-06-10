use chrono::{DateTime, Months, Utc};
use uuid::Uuid;

use crate::{
    dto::{Cursor, DEFAULT_PER_PAGE_LIMIT, MAX_PER_PAGE_LIMIT, OwnerName},
    error::UserError,
    util::cursor,
};

const DEFAULT_CONTRIBUTION_WINDOW_MONTHS: u32 = 12;

#[derive(Debug, Clone)]
pub struct ListUserContributedRepositoriesRequest {
    pub user_name: OwnerName,
    pub viewer_id: Option<Uuid>,
    pub from: DateTime<Utc>,
    pub cursor: Option<Cursor>,
    pub limit: u32,
}

impl ListUserContributedRepositoriesRequest {
    pub fn new(
        user_name: &str,
        viewer_id: Option<Uuid>,
        from: Option<DateTime<Utc>>,
        cursor: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Self, UserError> {
        let user_name = OwnerName::parse(user_name, "user name")?;
        let cursor = cursor.map(cursor::decode).transpose()?;
        Ok(Self {
            user_name,
            viewer_id,
            from: from
                .unwrap_or_else(|| Utc::now() - Months::new(DEFAULT_CONTRIBUTION_WINDOW_MONTHS)),
            cursor,
            limit: limit
                .unwrap_or(DEFAULT_PER_PAGE_LIMIT)
                .clamp(1, MAX_PER_PAGE_LIMIT),
        })
    }
}
