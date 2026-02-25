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
            s2_uri: self.s2_uri,
            name: self.name,
            command: self.command,
            status: self.status.into(),
            waits_for: self.waits_for,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

pub fn into_poll_api(task: TaskResponse) -> api::PollTaskResource {
    api::PollTaskResource {
        id: task.id,
        repo_owner: task.repo_owner,
        repo_name: task.repo_name,
        s2_uri: task.s2_uri,
        name: task.name,
        command: task.command,
        status: task.status.into(),
    }
}
