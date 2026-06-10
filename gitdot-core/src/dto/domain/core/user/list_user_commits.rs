use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    dto::{Cursor, DEFAULT_PER_PAGE_LIMIT, MAX_PER_PAGE_LIMIT, OwnerName},
    error::{InputError, UserError},
    util::cursor,
};

#[derive(Debug, Clone)]
pub struct ListUserCommitsRequest {
    pub user_name: OwnerName,
    pub viewer_id: Option<Uuid>,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    pub cursor: Option<Cursor>,
    pub limit: u32,
}

impl ListUserCommitsRequest {
    pub fn new(
        user_name: &str,
        viewer_id: Option<Uuid>,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
        cursor: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Self, UserError> {
        if to.is_some() && from.is_none() {
            return Err(InputError::new("date range", "`to` requires `from` to be set").into());
        }
        let user_name = OwnerName::parse(user_name, "user name")?;
        let cursor = cursor.map(cursor::decode).transpose()?;
        Ok(Self {
            user_name,
            viewer_id,
            from: from.unwrap_or(DateTime::<Utc>::UNIX_EPOCH),
            to: to.unwrap_or_else(Utc::now),
            cursor,
            limit: limit
                .unwrap_or(DEFAULT_PER_PAGE_LIMIT)
                .clamp(1, MAX_PER_PAGE_LIMIT),
        })
    }
}
