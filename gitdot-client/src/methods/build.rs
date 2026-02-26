use anyhow::Result;

use gitdot_api::endpoint::build::create_build::{CreateBuildRequest, CreateBuildResponse};

use crate::client::GitdotClient;

impl GitdotClient {
    pub async fn create_build(&self, request: CreateBuildRequest) -> Result<CreateBuildResponse> {
        self.post("ci/build".to_string(), request).await
    }
}
