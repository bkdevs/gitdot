use anyhow::Result;

use gitdot_api::endpoint::dag::create_dag::{CreateDagRequest, CreateDagResponse};

use crate::client::GitdotClient;

impl GitdotClient {
    pub async fn create_dag(&self, request: CreateDagRequest) -> Result<CreateDagResponse> {
        self.post("ci/dag".to_string(), request).await
    }
}
