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
            trigger: self.trigger,
            commit_sha: self.commit_sha,
            build_config: self.build_config,
            tasks: self.tasks.into_iter().map(|t| t.into_api()).collect(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
