use anyhow::Result;
use uuid::Uuid;

use gitdot_api::endpoint::task::{
    get_task::GetTaskResponse,
    poll_task::{PollTaskRequest, PollTaskResponse},
};

use crate::client::GitdotClient;

impl GitdotClient {
    pub async fn poll_task(&self, request: PollTaskRequest) -> Result<PollTaskResponse> {
        self.get("ci/task/poll".to_string(), request).await
    }

    pub async fn get_task(&self, id: Uuid) -> Result<GetTaskResponse> {
        self.get(format!("ci/task/{}", id), ()).await
    }
}
