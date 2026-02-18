use gitdot_api::resource::runner as api;
use gitdot_core::dto::RunnerResponse;

use super::IntoApi;

impl IntoApi for RunnerResponse {
    type ApiType = api::RunnerResource;
    fn into_api(self) -> Self::ApiType {
        api::RunnerResource {
            id: self.id,
            name: self.name,
            owner_id: self.owner_id,
            owner_type: self.owner_type.into(),
            created_at: self.created_at,
        }
    }
}
