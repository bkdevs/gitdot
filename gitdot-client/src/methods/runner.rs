use anyhow::Result;

use gitdot_api::endpoint::runner::{
    create_runner::{CreateRunnerRequest, CreateRunnerResponse},
    create_runner_token::CreateRunnerTokenResponse,
    delete_runner::DeleteRunnerResponse,
    get_runner::GetRunnerResponse,
};

use crate::client::GitdotClient;

impl GitdotClient {
    pub async fn verify_runner(&self) -> Result<()> {
        self.post("ci/runner/verify".to_string(), ()).await
    }

    pub async fn create_runner(
        &self,
        owner: &str,
        request: CreateRunnerRequest,
    ) -> Result<CreateRunnerResponse> {
        self.post(format!("ci/runner/{}", owner), request).await
    }

    pub async fn get_runner(&self, owner: &str, name: &str) -> Result<GetRunnerResponse> {
        self.get(format!("ci/runner/{}/{}", owner, name), ()).await
    }

    pub async fn create_runner_token(
        &self,
        owner: &str,
        name: &str,
    ) -> Result<CreateRunnerTokenResponse> {
        self.post(format!("ci/runner/{}/{}/token", owner, name), ())
            .await
    }

    pub async fn delete_runner(&self, owner: &str, name: &str) -> Result<DeleteRunnerResponse> {
        self.delete(format!("ci/runner/{}/{}", owner, name), ())
            .await
    }
}
