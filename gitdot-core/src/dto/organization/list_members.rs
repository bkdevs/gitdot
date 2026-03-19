use crate::{dto::OwnerName, error::OrganizationError, model::OrganizationRole};

#[derive(Debug, Clone)]
pub struct ListMembersRequest {
    pub org_name: OwnerName,
    pub role: Option<OrganizationRole>,
}

impl ListMembersRequest {
    pub fn new(org_name: &str, role: Option<&str>) -> Result<Self, OrganizationError> {
        let role = role
            .map(|r| match r {
                "admin" => Ok(OrganizationRole::Admin),
                "member" => Ok(OrganizationRole::Member),
                _ => Err(OrganizationError::InvalidRole(r.to_string())),
            })
            .transpose()?;

        Ok(Self {
            org_name: OwnerName::try_new(org_name)
                .map_err(|e| OrganizationError::InvalidOrganizationName(e.to_string()))?,
            role,
        })
    }
}
