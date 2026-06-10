use crate::{
    dto::{Cursor, DEFAULT_PER_PAGE_LIMIT, MAX_PER_PAGE_LIMIT, OwnerName},
    error::UserError,
    util::cursor,
};

#[derive(Debug, Clone)]
pub struct ListUserOrganizationsRequest {
    pub user_name: OwnerName,
    pub cursor: Option<Cursor>,
    pub limit: u32,
}

impl ListUserOrganizationsRequest {
    pub fn new(
        user_name: &str,
        cursor: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Self, UserError> {
        let user_name = OwnerName::parse(user_name, "user name")?;
        let cursor = cursor.map(cursor::decode).transpose()?;
        Ok(Self {
            user_name,
            cursor,
            limit: limit
                .unwrap_or(DEFAULT_PER_PAGE_LIMIT)
                .clamp(1, MAX_PER_PAGE_LIMIT),
        })
    }
}
