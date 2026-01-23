use super::OrganizationName;

#[derive(Debug, Clone)]
pub struct GetOrganizationRequest {
    pub org_name: OrganizationName,
}

impl GetOrganizationRequest {
    pub fn new(org_name: &str) -> Self {
        Self {
            org_name: OrganizationName::try_new(org_name).unwrap(),
        }
    }
}
