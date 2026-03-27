use async_trait::async_trait;

use crate::{
    dto::{
        CreateWebhookRequest, DeleteWebhookRequest, GetWebhookRequest, ListWebhooksRequest,
        UpdateWebhookRequest, WebhookResponse,
    },
    error::WebhookError,
    repository::{
        RepositoryRepository, RepositoryRepositoryImpl, WebhookRepository, WebhookRepositoryImpl,
    },
};

#[async_trait]
pub trait WebhookService: Send + Sync + 'static {
    async fn create_webhook(
        &self,
        request: CreateWebhookRequest,
    ) -> Result<WebhookResponse, WebhookError>;

    async fn list_webhooks(
        &self,
        request: ListWebhooksRequest,
    ) -> Result<Vec<WebhookResponse>, WebhookError>;

    async fn get_webhook(
        &self,
        request: GetWebhookRequest,
    ) -> Result<WebhookResponse, WebhookError>;

    async fn update_webhook(
        &self,
        request: UpdateWebhookRequest,
    ) -> Result<WebhookResponse, WebhookError>;

    async fn delete_webhook(&self, request: DeleteWebhookRequest) -> Result<(), WebhookError>;
}

#[derive(Debug, Clone)]
pub struct WebhookServiceImpl<W, R>
where
    W: WebhookRepository,
    R: RepositoryRepository,
{
    webhook_repo: W,
    repo_repo: R,
}

impl WebhookServiceImpl<WebhookRepositoryImpl, RepositoryRepositoryImpl> {
    pub fn new(webhook_repo: WebhookRepositoryImpl, repo_repo: RepositoryRepositoryImpl) -> Self {
        Self {
            webhook_repo,
            repo_repo,
        }
    }
}

#[crate::instrument_all]
#[async_trait]
impl<W, R> WebhookService for WebhookServiceImpl<W, R>
where
    W: WebhookRepository,
    R: RepositoryRepository,
{
    async fn create_webhook(
        &self,
        request: CreateWebhookRequest,
    ) -> Result<WebhookResponse, WebhookError> {
        let owner = request.owner_name.as_ref();
        let repo = request.repo_name.as_ref();

        let repository = self
            .repo_repo
            .get(owner, repo)
            .await?
            .ok_or_else(|| WebhookError::RepositoryNotFound(format!("{owner}/{repo}")))?;

        let webhook = self
            .webhook_repo
            .create(
                repository.id,
                &request.url,
                &request.secret,
                &request.events,
            )
            .await?;

        Ok(webhook.into())
    }

    async fn list_webhooks(
        &self,
        request: ListWebhooksRequest,
    ) -> Result<Vec<WebhookResponse>, WebhookError> {
        let owner = request.owner_name.as_ref();
        let repo = request.repo_name.as_ref();

        let repository = self
            .repo_repo
            .get(owner, repo)
            .await?
            .ok_or_else(|| WebhookError::RepositoryNotFound(format!("{owner}/{repo}")))?;

        let webhooks = self.webhook_repo.list_by_repo(repository.id).await?;

        Ok(webhooks.into_iter().map(Into::into).collect())
    }

    async fn get_webhook(
        &self,
        request: GetWebhookRequest,
    ) -> Result<WebhookResponse, WebhookError> {
        let owner = request.owner_name.as_ref();
        let repo = request.repo_name.as_ref();

        let repository = self
            .repo_repo
            .get(owner, repo)
            .await?
            .ok_or_else(|| WebhookError::RepositoryNotFound(format!("{owner}/{repo}")))?;

        let webhook = self
            .webhook_repo
            .get(request.webhook_id)
            .await?
            .ok_or_else(|| WebhookError::NotFound(request.webhook_id.to_string()))?;

        if webhook.repository_id != repository.id {
            return Err(WebhookError::NotFound(request.webhook_id.to_string()));
        }

        Ok(webhook.into())
    }

    async fn update_webhook(
        &self,
        request: UpdateWebhookRequest,
    ) -> Result<WebhookResponse, WebhookError> {
        let owner = request.owner_name.as_ref();
        let repo = request.repo_name.as_ref();

        let repository = self
            .repo_repo
            .get(owner, repo)
            .await?
            .ok_or_else(|| WebhookError::RepositoryNotFound(format!("{owner}/{repo}")))?;

        let existing = self
            .webhook_repo
            .get(request.webhook_id)
            .await?
            .ok_or_else(|| WebhookError::NotFound(request.webhook_id.to_string()))?;

        if existing.repository_id != repository.id {
            return Err(WebhookError::NotFound(request.webhook_id.to_string()));
        }

        let webhook = self
            .webhook_repo
            .update(
                request.webhook_id,
                request.url.as_ref().map(|u| u.as_ref()),
                request.secret.as_deref(),
                request.events.as_deref(),
            )
            .await?;

        Ok(webhook.into())
    }

    async fn delete_webhook(&self, request: DeleteWebhookRequest) -> Result<(), WebhookError> {
        let owner = request.owner_name.as_ref();
        let repo = request.repo_name.as_ref();

        let repository = self
            .repo_repo
            .get(owner, repo)
            .await?
            .ok_or_else(|| WebhookError::RepositoryNotFound(format!("{owner}/{repo}")))?;

        let existing = self
            .webhook_repo
            .get(request.webhook_id)
            .await?
            .ok_or_else(|| WebhookError::NotFound(request.webhook_id.to_string()))?;

        if existing.repository_id != repository.id {
            return Err(WebhookError::NotFound(request.webhook_id.to_string()));
        }

        self.webhook_repo.delete(request.webhook_id).await?;

        Ok(())
    }
}
