use crate::{
    dto::{Cursor, DEFAULT_PER_PAGE_LIMIT, MAX_PER_PAGE_LIMIT, OwnerName},
    error::{InputError, OrganizationError},
    model::OrganizationRole,
    util::cursor,
};

#[derive(Debug, Clone)]
pub struct ListMembersRequest {
    pub org_name: OwnerName,
    pub role: Option<OrganizationRole>,
    pub cursor: Option<Cursor>,
    pub limit: u32,
}

impl ListMembersRequest {
    pub fn new(
        org_name: &str,
        role: Option<&str>,
        cursor: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Self, OrganizationError> {
        let role = role
            .map(|r| match r {
                "admin" => Ok(OrganizationRole::Admin),
                "member" => Ok(OrganizationRole::Member),
                _ => Err(OrganizationError::Input(InputError::new("role", r))),
            })
            .transpose()?;
        let org_name =
            OwnerName::parse(org_name, "organization name")?;
        let cursor = cursor.map(cursor::decode).transpose()?;

        Ok(Self {
            org_name,
            role,
            cursor,
            limit: limit
                .unwrap_or(DEFAULT_PER_PAGE_LIMIT)
                .clamp(1, MAX_PER_PAGE_LIMIT),
        })
    }
}
