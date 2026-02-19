use gitdot_api::resource::organization as api;
use gitdot_core::{
    dto::{OrganizationMemberResponse, OrganizationResponse},
    model::organization::OrganizationRole,
};

use super::IntoApi;

impl IntoApi for OrganizationResponse {
    type ApiType = api::OrganizationResource;
    fn into_api(self) -> Self::ApiType {
        api::OrganizationResource {
            id: self.id,
            name: self.name,
            created_at: self.created_at,
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
            role: match self.role {
                OrganizationRole::Admin => "admin".to_string(),
                OrganizationRole::Member => "member".to_string(),
            },
            created_at: self.created_at,
        }
    }
}
