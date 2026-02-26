use gitdot_api::resource::user as api;
use gitdot_core::dto::UserResponse;

use super::IntoApi;

impl IntoApi for UserResponse {
    type ApiType = api::UserResource;
    fn into_api(self) -> Self::ApiType {
        api::UserResource {
            id: self.id,
            name: self.name,
            email: self.email,
            created_at: self.created_at,
        }
    }
}
