use gitdot_api::resource::runner as api;
use gitdot_core::dto::{CreateRunnerTokenResponse, RunnerResponse};

use super::IntoApi;

impl IntoApi for RunnerResponse {
    type ApiType = api::RunnerResource;
    fn into_api(self) -> Self::ApiType {
        api::RunnerResource {
            id: self.id,
            name: self.name,
            owner_id: self.owner_id,
            owner_name: self.owner_name,
            owner_type: self.owner_type.into(),
            last_verified: self.last_verified,
            created_at: self.created_at,
        }
    }
}

impl IntoApi for CreateRunnerTokenResponse {
    type ApiType = api::RunnerTokenResource;
    fn into_api(self) -> Self::ApiType {
        api::RunnerTokenResource { token: self.token }
    }
}
