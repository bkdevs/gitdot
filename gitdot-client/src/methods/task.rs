use anyhow::Result;
use uuid::Uuid;

use gitdot_api::endpoint::task::{
    get_task::GetTaskResponse,
    poll_task::{PollTaskRequest, PollTaskResponse},
    update_task::{UpdateTaskRequest, UpdateTaskResponse},
};

use crate::client::GitdotClient;

impl GitdotClient {
    pub async fn poll_task(&self, request: PollTaskRequest) -> Result<PollTaskResponse> {
        self.get("ci/task/poll".to_string(), request).await
    }

    pub async fn get_task(&self, id: Uuid) -> Result<GetTaskResponse> {
        self.get(format!("ci/task/{}", id), ()).await
    }

    pub async fn update_task(&self, id: Uuid, status: &str) -> Result<UpdateTaskResponse> {
        let request = UpdateTaskRequest {
            status: status.to_string(),
        };
        self.patch(format!("ci/task/{}", id), request).await
    }
}
