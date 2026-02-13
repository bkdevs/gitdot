use super::IntoApi;

use api::resource::task as api;
use gitdot_core::dto::TaskResponse;

impl IntoApi for TaskResponse {
    type ApiType = api::TaskResource;
    fn into_api(self) -> Self::ApiType {
        api::TaskResource {
            id: self.id,
            repo_owner: self.repo_owner,
            repo_name: self.repo_name,
            script: self.script,
            status: self.status.into(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
