use super::IntoApi;

use api::resource::dag as api;
use gitdot_core::dto::DagResponse;

impl IntoApi for DagResponse {
    type ApiType = api::DagResource;
    fn into_api(self) -> Self::ApiType {
        api::DagResource {
            id: self.id,
            repo_owner: self.repo_owner,
            repo_name: self.repo_name,
            task_ids: self.task_ids,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
