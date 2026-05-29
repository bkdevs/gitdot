use gitdot_api::resource::organization as api;
use gitdot_core::{
    dto::{OrganizationMemberResponse, OrganizationResponse},
    model::OrganizationRole,
};

use super::IntoApi;

impl IntoApi for OrganizationResponse {
    type ApiType = api::OrganizationResource;
    fn into_api(self) -> Self::ApiType {
        api::OrganizationResource {
            id: self.id,
            name: self.name,
            display_name: self.display_name,
            location: self.location,
            readme: self.readme,
            links: self.links,
            created_at: self.created_at,
            image_updated_at: self.image_updated_at,
            members: self.members.into_api(),
        }
    }
}

impl IntoApi for OrganizationMemberResponse {
    type ApiType = api::OrganizationMemberResource;
    fn into_api(self) -> Self::ApiType {
        api::OrganizationMemberResource {
            id: self.id,
            user_id: self.user_id,
            user_name: self.user_name,
            role: match self.role {
                OrganizationRole::Admin => "admin".to_string(),
                OrganizationRole::Member => "member".to_string(),
            },
            role_description: self.role_description,
            created_at: self.created_at,
            image_updated_at: self.image_updated_at,
        }
    }
}
