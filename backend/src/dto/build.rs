use gitdot_api::resource::build as api;
use gitdot_core::dto::BuildResponse;

use super::IntoApi;

impl IntoApi for BuildResponse {
    type ApiType = api::BuildResource;
    fn into_api(self) -> Self::ApiType {
        api::BuildResource {
            id: self.id,
            repo_owner: self.repo_owner,
            repo_name: self.repo_name,
            task_dependencies: self.task_dependencies,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
