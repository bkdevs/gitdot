use crate::{
    dto::OwnerName,
    error::{InputError, OrganizationError},
    model::OrganizationRole,
};

#[derive(Debug, Clone)]
pub struct AddMemberRequest {
    pub org_name: OwnerName,
    pub user_name: OwnerName,
    pub role: OrganizationRole,
    pub role_description: Option<String>,
}

impl AddMemberRequest {
    pub fn new(
        org_name: &str,
        user_name: &str,
        role: &str,
        role_description: Option<String>,
    ) -> Result<Self, OrganizationError> {
        let role = match role {
            "admin" => OrganizationRole::Admin,
            "member" => OrganizationRole::Member,
            _ => return Err(InputError::new("role", role).into()),
        };

        Ok(Self {
            org_name: OwnerName::parse(org_name, "organization name")?,
            user_name: OwnerName::parse(user_name, "user name")?,
            role,
            role_description,
        })
    }
}
