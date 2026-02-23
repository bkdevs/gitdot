use gitdot_api::resource::dag as api;
use gitdot_core::dto::DagResponse;

use super::IntoApi;

impl IntoApi for DagResponse {
    type ApiType = api::DagResource;
    fn into_api(self) -> Self::ApiType {
        api::DagResource {
            id: self.id,
            repo_owner: self.repo_owner,
            repo_name: self.repo_name,
            task_dependencies: self.task_dependencies,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
