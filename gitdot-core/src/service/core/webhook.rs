use async_trait::async_trait;
use chrono::Utc;

use crate::{
    client::{Git2Client, GitClient, KafkaClient, KafkaClientImpl},
    dto::{
        CreateWebhookRequest, DeleteWebhookRequest, GetWebhookRequest, ListWebhooksRequest,
        PublishRepoPushRequest, RepoPushCommit, RepoPushEvent, UpdateWebhookRequest,
        WebhookResponse,
    },
    error::{NotFoundError, OptionNotFoundExt, WebhookError},
    repository::{
        RepositoryRepository, RepositoryRepositoryImpl, UserRepository, UserRepositoryImpl,
        WebhookRepository, WebhookRepositoryImpl,
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

    async fn publish_repo_push(&self, request: PublishRepoPushRequest) -> Result<(), WebhookError>;
}

#[derive(Debug, Clone)]
pub struct WebhookServiceImpl<W, R, U, G, K>
where
    W: WebhookRepository,
    R: RepositoryRepository,
    U: UserRepository,
    G: GitClient,
    K: KafkaClient,
{
    webhook_repo: W,
    repo_repo: R,
    user_repo: U,
    git_client: G,
    kafka_client: K,
}

impl
    WebhookServiceImpl<
        WebhookRepositoryImpl,
        RepositoryRepositoryImpl,
        UserRepositoryImpl,
        Git2Client,
        KafkaClientImpl,
    >
{
    pub fn new(
        webhook_repo: WebhookRepositoryImpl,
        repo_repo: RepositoryRepositoryImpl,
        user_repo: UserRepositoryImpl,
        git_client: Git2Client,
        kafka_client: KafkaClientImpl,
    ) -> Self {
        Self {
            webhook_repo,
            repo_repo,
            user_repo,
            git_client,
            kafka_client,
        }
    }
}

#[crate::instrument_all]
#[async_trait]
impl<W, R, U, G, K> WebhookService for WebhookServiceImpl<W, R, U, G, K>
where
    W: WebhookRepository,
    R: RepositoryRepository,
    U: UserRepository,
    G: GitClient,
    K: KafkaClient,
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
            .or_not_found("repository", format!("{owner}/{repo}"))?;

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
            .or_not_found("repository", format!("{owner}/{repo}"))?;

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
            .or_not_found("repository", format!("{owner}/{repo}"))?;

        let webhook = self
            .webhook_repo
            .get(request.webhook_id)
            .await?
            .or_not_found("webhook", request.webhook_id)?;

        if webhook.repository_id != repository.id {
            return Err(NotFoundError::new("webhook", request.webhook_id).into());
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
            .or_not_found("repository", format!("{owner}/{repo}"))?;

        let existing = self
            .webhook_repo
            .get(request.webhook_id)
            .await?
            .or_not_found("webhook", request.webhook_id)?;

        if existing.repository_id != repository.id {
            return Err(NotFoundError::new("webhook", request.webhook_id).into());
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
            .or_not_found("repository", format!("{owner}/{repo}"))?;

        let existing = self
            .webhook_repo
            .get(request.webhook_id)
            .await?
            .or_not_found("webhook", request.webhook_id)?;

        if existing.repository_id != repository.id {
            return Err(NotFoundError::new("webhook", request.webhook_id).into());
        }

        self.webhook_repo.delete(request.webhook_id).await?;

        Ok(())
    }

    async fn publish_repo_push(&self, request: PublishRepoPushRequest) -> Result<(), WebhookError> {
        let pusher = self
            .user_repo
            .get_by_id(request.pusher_id)
            .await?
            .or_not_found("user", request.pusher_id)?;

        let git_commits = self
            .git_client
            .rev_list(
                &request.owner,
                &request.repo,
                &request.old_sha,
                &request.new_sha,
            )
            .await?;

        let commits = git_commits
            .into_iter()
            .map(|c| RepoPushCommit {
                sha: c.sha,
                message: c.message,
            })
            .collect();

        let event = RepoPushEvent {
            owner: request.owner,
            repo: request.repo,
            ref_name: request.ref_name,
            old_sha: request.old_sha,
            new_sha: request.new_sha,
            pusher_id: request.pusher_id,
            pusher_name: pusher.name,
            commits,
            pushed_at: Utc::now(),
        };

        self.kafka_client.publish_repo_push(event).await?;

        Ok(())
    }
}
