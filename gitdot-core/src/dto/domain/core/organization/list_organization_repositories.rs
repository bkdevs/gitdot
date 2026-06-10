use uuid::Uuid;

use crate::{
    dto::{Cursor, DEFAULT_PER_PAGE_LIMIT, MAX_PER_PAGE_LIMIT, OwnerName},
    error::OrganizationError,
    util::cursor,
};

#[derive(Debug, Clone)]
pub struct ListOrganizationRepositoriesRequest {
    pub org_name: OwnerName,
    pub viewer_id: Option<Uuid>,
    pub cursor: Option<Cursor>,
    pub limit: u32,
}

impl ListOrganizationRepositoriesRequest {
    pub fn new(
        org_name: &str,
        cursor: Option<&str>,
        limit: Option<u32>,
        viewer_id: Option<Uuid>,
    ) -> Result<Self, OrganizationError> {
        let org_name = OwnerName::parse(org_name, "organization name")?;
        let cursor = cursor.map(cursor::decode).transpose()?;
        Ok(Self {
            org_name,
            viewer_id,
            cursor,
            limit: limit
                .unwrap_or(DEFAULT_PER_PAGE_LIMIT)
                .clamp(1, MAX_PER_PAGE_LIMIT),
        })
    }
}
