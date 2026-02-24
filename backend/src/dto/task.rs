use gitdot_api::resource::task as api;
use gitdot_core::dto::TaskResponse;

use super::IntoApi;

impl IntoApi for TaskResponse {
    type ApiType = api::TaskResource;
    fn into_api(self) -> Self::ApiType {
        api::TaskResource {
            id: self.id,
            repo_owner: self.repo_owner,
            repo_name: self.repo_name,
            build_id: self.build_id,
            script: self.script,
            status: self.status.into(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
