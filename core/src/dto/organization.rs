mod add_member;
mod create_organization;
mod get_organization;
mod list_organization_repositories;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::organization::Organization;

pub use add_member::{AddMemberRequest, OrganizationMemberResponse};
pub use create_organization::CreateOrganizationRequest;
pub use get_organization::GetOrganizationRequest;
pub use list_organization_repositories::ListOrganizationRepositoriesRequest;

#[derive(Debug, Clone)]
pub struct OrganizationResponse {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl From<Organization> for OrganizationResponse {
    fn from(org: Organization) -> Self {
        Self {
            id: org.id,
            name: org.name,
            created_at: org.created_at,
        }
    }
}
