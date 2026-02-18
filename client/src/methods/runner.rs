use anyhow::Result;
use uuid::Uuid;

use gitdot_api::endpoint::runner::{
    create_runner::{CreateRunnerRequest, CreateRunnerResponse},
    create_runner_token::CreateRunnerTokenResponse,
    delete_runner::{DeleteRunnerRequest, DeleteRunnerResponse},
    get_runner::{GetRunnerRequest, GetRunnerResponse},
};

use crate::client::GitdotClient;

impl GitdotClient {
    pub async fn create_runner(
        &self,
        request: CreateRunnerRequest,
    ) -> Result<CreateRunnerResponse> {
        self.post("ci/runner".to_string(), request).await
    }

    pub async fn get_runner(
        &self,
        name: &str,
        request: GetRunnerRequest,
    ) -> Result<GetRunnerResponse> {
        self.get(format!("ci/runner/{}", name), request).await
    }

    pub async fn create_runner_token(&self, id: Uuid) -> Result<CreateRunnerTokenResponse> {
        self.post(format!("ci/runner/{}/token", id), ()).await
    }

    pub async fn verify_runner(&self, id: Uuid) -> Result<()> {
        self.post(format!("ci/runner/{}/verify", id), ()).await
    }

    pub async fn delete_runner(
        &self,
        id: Uuid,
        request: DeleteRunnerRequest,
    ) -> Result<DeleteRunnerResponse> {
        self.delete(format!("ci/runner/{}", id), request).await
    }
}
