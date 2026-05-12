mod add_member;
mod create_organization;
mod get_organization;
mod list_members;
mod list_organization_repositories;
mod update_member;
mod update_organization;
mod update_organization_image;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::Organization;

pub use add_member::{AddMemberRequest, OrganizationMemberResponse};
pub use create_organization::CreateOrganizationRequest;
pub use get_organization::GetOrganizationRequest;
pub use list_members::ListMembersRequest;
pub use list_organization_repositories::ListOrganizationRepositoriesRequest;
pub use update_member::UpdateOrganizationMemberRequest;
pub use update_organization::UpdateOrganizationRequest;
pub use update_organization_image::UpdateOrganizationImageRequest;

#[derive(Debug, Clone)]
pub struct OrganizationResponse {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub location: Option<String>,
    pub readme: Option<String>,
    pub links: Vec<String>,
    pub display_name: Option<String>,
}

impl From<Organization> for OrganizationResponse {
    fn from(org: Organization) -> Self {
        Self {
            id: org.id,
            name: org.name,
            created_at: org.created_at,
            location: org.location,
            readme: org.readme,
            links: org.links,
            display_name: org.display_name,
        }
    }
}
