use uuid::Uuid;

use crate::{
    dto::{Cursor, DEFAULT_PER_PAGE_LIMIT, MAX_PER_PAGE_LIMIT, OwnerName},
    error::UserError,
    util::cursor,
};

#[derive(Debug, Clone)]
pub struct ListUserReviewsRequest {
    pub user_name: OwnerName,
    pub viewer_id: Option<Uuid>,
    pub status: Option<String>,
    pub owner: Option<String>,
    pub repo: Option<String>,
    pub cursor: Option<Cursor>,
    pub limit: u32,
}

impl ListUserReviewsRequest {
    pub fn new(
        user_name: &str,
        viewer_id: Option<Uuid>,
        status: Option<String>,
        owner: Option<String>,
        repo: Option<String>,
        cursor: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Self, UserError> {
        let user_name = OwnerName::parse(user_name, "user name")?;
        let cursor = cursor.map(cursor::decode).transpose()?;
        Ok(Self {
            user_name,
            viewer_id,
            status,
            owner,
            repo,
            cursor,
            limit: limit
                .unwrap_or(DEFAULT_PER_PAGE_LIMIT)
                .clamp(1, MAX_PER_PAGE_LIMIT),
        })
    }
}
