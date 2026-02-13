use api::resource::UserResource;
use gitdot_core::dto::UserResponse;

use super::IntoApi;

impl IntoApi for UserResponse {
    type ApiType = UserResource;
    fn into_api(self) -> Self::ApiType {
        UserResource {
            id: self.id,
            name: self.name,
            email: self.email,
            created_at: self.created_at,
        }
    }
}
