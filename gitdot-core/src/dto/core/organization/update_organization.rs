use crate::{
    dto::OwnerName,
    error::{InputError, OrganizationError},
};

#[derive(Debug, Clone)]
pub struct UpdateOrganizationRequest {
    pub org_name: OwnerName,
    pub location: Option<String>,
    pub readme: Option<String>,
    pub links: Option<Vec<String>>,
    pub display_name: Option<String>,
}

impl UpdateOrganizationRequest {
    pub fn new(
        org_name: &str,
        location: Option<String>,
        readme: Option<String>,
        links: Option<Vec<String>>,
        display_name: Option<String>,
    ) -> Result<Self, OrganizationError> {
        Ok(Self {
            org_name: OwnerName::try_new(org_name)
                .map_err(|e| InputError::new("organization name", e))?,
            location,
            readme,
            links,
            display_name,
        })
    }
}
