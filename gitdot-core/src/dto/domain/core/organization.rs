mod add_member;
mod create_organization;
mod follow_organization;
mod get_organization;
mod list_organization_repositories;
mod list_organizations;
mod unfollow_organization;
mod update_member;
mod update_organization;
mod update_organization_image;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::{Organization, OrganizationMember, OrganizationRole};

pub use add_member::AddMemberRequest;
pub use create_organization::CreateOrganizationRequest;
pub use follow_organization::FollowOrganizationRequest;
pub use get_organization::GetOrganizationRequest;
pub use list_organization_repositories::ListOrganizationRepositoriesRequest;
pub use list_organizations::ListOrganizationsRequest;
pub use unfollow_organization::UnfollowOrganizationRequest;
pub use update_member::UpdateOrganizationMemberRequest;
pub use update_organization::UpdateOrganizationRequest;
pub use update_organization_image::UpdateOrganizationImageRequest;

#[derive(Debug, Clone)]
pub struct OrganizationResponse {
    pub id: Uuid,
    pub name: String,

    pub display_name: Option<String>,
    pub location: Option<String>,
    pub readme: Option<String>,
    pub links: Vec<String>,

    pub created_at: DateTime<Utc>,
    pub image_updated_at: DateTime<Utc>,
    pub followers: i64,
    pub user_follow: bool,

    pub members: Option<Vec<OrganizationMemberResponse>>,
}

impl From<Organization> for OrganizationResponse {
    fn from(org: Organization) -> Self {
        Self {
            id: org.id,
            name: org.name,
            display_name: org.display_name,
            location: org.location,
            readme: org.readme,
            links: org.links,
            created_at: org.created_at,
            image_updated_at: org.image_updated_at,
            followers: org.followers,
            user_follow: org.user_follow,
            members: org
                .members
                .map(|members| members.into_iter().map(Into::into).collect()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OrganizationMemberResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,

    pub role: OrganizationRole,
    pub role_description: Option<String>,

    pub created_at: DateTime<Utc>,
    pub image_updated_at: DateTime<Utc>,
}

impl From<OrganizationMember> for OrganizationMemberResponse {
    fn from(member: OrganizationMember) -> Self {
        Self {
            id: member.id,
            user_id: member.user_id,
            user_name: member.user_name,
            role: member.role,
            role_description: member.role_description,
            created_at: member.created_at,
            image_updated_at: member.image_updated_at,
        }
    }
}
