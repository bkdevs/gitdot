use async_trait::async_trait;

use crate::{dto::ProcessGithubPushRequest, error::WebhookError};

#[async_trait]
pub trait GithubWebhookService: Send + Sync + 'static {
    async fn process_github_push(
        &self,
        request: ProcessGithubPushRequest,
    ) -> Result<(), WebhookError>;
}

#[derive(Debug, Clone, Default)]
pub struct GithubWebhookServiceImpl;

impl GithubWebhookServiceImpl {
    pub fn new() -> Self {
        Self
    }
}

#[crate::instrument_all]
#[async_trait]
impl GithubWebhookService for GithubWebhookServiceImpl {
    async fn process_github_push(
        &self,
        request: ProcessGithubPushRequest,
    ) -> Result<(), WebhookError> {
        tracing::info!(
            owner = %request.repository.owner.login,
            repo = %request.repository.name,
            ref_name = %request.ref_name,
            installation_id = request.installation.id,
            commits = request.commits.len(),
            "received github push event",
        );
        Ok(())
    }
}
