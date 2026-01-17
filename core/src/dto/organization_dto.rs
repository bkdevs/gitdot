use nutype::nutype;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateOrganizationRequest {
    pub name: OrganizationName,
}

impl CreateOrganizationRequest {
    pub fn new(org_name: String) -> Self {
        Self {
            name: OrganizationName::try_new(org_name).unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FindOrganizationByNameRequest {
    pub name: OrganizationName,
}

impl FindOrganizationByNameRequest {
    pub fn new(org_name: String) -> Self {
        Self {
            name: OrganizationName::try_new(org_name).unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AddOrganizationMemberRequest {
    pub user_id: Uuid,
    pub organization_id: Uuid,
}

#[nutype(
    sanitize(trim, lowercase),
    validate(not_empty, len_char_max = 100),
    derive(
        Debug,
        Clone,
        Serialize,
        Deserialize,
        PartialEq,
        Eq,
        TryFrom,
        AsRef,
        Deref
    )
)]
pub struct OrganizationName(String);
