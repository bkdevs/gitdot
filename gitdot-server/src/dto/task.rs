use gitdot_api::resource::task as api;
use gitdot_core::dto::{TaskLogResponse, TaskResponse};

use super::IntoApi;

impl IntoApi for TaskLogResponse {
    type ApiType = api::TaskLogResource;
    fn into_api(self) -> Self::ApiType {
        api::TaskLogResource {
            seq_num: self.seq_num,
            timestamp: self.timestamp,
            body: self.body,
            stream: self.stream,
            finished: self.finished,
        }
    }
}

impl IntoApi for TaskResponse {
    type ApiType = api::TaskResource;
    fn into_api(self) -> Self::ApiType {
        api::TaskResource {
            id: self.id,
            repository_id: self.repository_id,
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
