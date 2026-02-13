use super::IntoApi;

use api::resource::runner as api;
use gitdot_core::dto::{CreateRunnerResponse, RunnerResponse};

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

impl IntoApi for CreateRunnerResponse {
    type ApiType = api::CreateRunnerResource;
    fn into_api(self) -> Self::ApiType {
        api::CreateRunnerResource {
            token: self.token,
        }
    }
}
