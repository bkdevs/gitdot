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
            created_at: self.created_at,
            location: self.location,
            readme: self.readme,
            links: self.links,
            display_name: self.display_name,
        }
    }
}

impl IntoApi for OrganizationMemberResponse {
    type ApiType = api::OrganizationMemberResource;
    fn into_api(self) -> Self::ApiType {
        api::OrganizationMemberResource {
            id: self.id,
            user_id: self.user_id,
            organization_id: self.organization_id,
            org_name: self.org_name,
            role: match self.role {
                OrganizationRole::Admin => "admin".to_string(),
                OrganizationRole::Member => "member".to_string(),
            },
            role_description: self.role_description,
            created_at: self.created_at,
            user_name: self.user_name,
        }
    }
}
