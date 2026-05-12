use async_trait::async_trait;

use crate::{
    client::{SlackBotClient, SlackBotClientImpl},
    dto::{
        ListSlackWebhooksRequest, NotifyRepoPushRequest, SlackWebhookResponse,
        SubscribeSlackWebhookRequest, UnsubscribeSlackWebhookRequest,
    },
    error::{NotFoundError, OptionNotFoundExt, WebhookError},
    model::WebhookEventType,
    repository::{
        RepositoryRepository, RepositoryRepositoryImpl, SlackWebhookRepository,
        SlackWebhookRepositoryImpl,
    },
};

#[async_trait]
pub trait SlackWebhookService: Send + Sync + 'static {
    async fn subscribe_slack_webhook(
        &self,
        request: SubscribeSlackWebhookRequest,
    ) -> Result<SlackWebhookResponse, WebhookError>;

    async fn unsubscribe_slack_webhook(
        &self,
        request: UnsubscribeSlackWebhookRequest,
    ) -> Result<(), WebhookError>;

    async fn list_slack_webhooks(
        &self,
        request: ListSlackWebhooksRequest,
    ) -> Result<Vec<SlackWebhookResponse>, WebhookError>;

    async fn notify_slack_of_repo_push(
        &self,
        request: NotifyRepoPushRequest,
    ) -> Result<(), WebhookError>;
}

#[derive(Debug, Clone)]
pub struct SlackWebhookServiceImpl<SW, R, SBC>
where
    SW: SlackWebhookRepository,
    R: RepositoryRepository,
    SBC: SlackBotClient,
{
    slack_webhook_repo: SW,
    repo_repo: R,
    slack_bot_client: SBC,
}

impl
    SlackWebhookServiceImpl<
        SlackWebhookRepositoryImpl,
        RepositoryRepositoryImpl,
        SlackBotClientImpl,
    >
{
    pub fn new(
        slack_webhook_repo: SlackWebhookRepositoryImpl,
        repo_repo: RepositoryRepositoryImpl,
        slack_bot_client: SlackBotClientImpl,
    ) -> Self {
        Self {
            slack_webhook_repo,
            repo_repo,
            slack_bot_client,
        }
    }
}

#[crate::instrument_all]
#[async_trait]
impl<SW, R, SBC> SlackWebhookService for SlackWebhookServiceImpl<SW, R, SBC>
where
    SW: SlackWebhookRepository,
    R: RepositoryRepository,
    SBC: SlackBotClient,
{
    async fn subscribe_slack_webhook(
        &self,
        request: SubscribeSlackWebhookRequest,
    ) -> Result<SlackWebhookResponse, WebhookError> {
        let owner = request.owner_name.as_ref();
        let repo = request.repo_name.as_ref();

        let repository = self
            .repo_repo
            .get(owner, repo)
            .await?
            .or_not_found("repository", format!("{owner}/{repo}"))?;

        // TODO: support configurable event subscriptions; default to push only.
        let events = vec![WebhookEventType::Push];
        let webhook = self
            .slack_webhook_repo
            .create(
                request.user_id,
                repository.id,
                &events,
                &request.slack_user_id,
                &request.slack_team_id,
                &request.slack_channel_id,
            )
            .await?;

        Ok(webhook.into())
    }

    async fn unsubscribe_slack_webhook(
        &self,
        request: UnsubscribeSlackWebhookRequest,
    ) -> Result<(), WebhookError> {
        let owner = request.owner_name.as_ref();
        let repo = request.repo_name.as_ref();

        let repository = self
            .repo_repo
            .get(owner, repo)
            .await?
            .or_not_found("repository", format!("{owner}/{repo}"))?;

        let webhook = self
            .slack_webhook_repo
            .get(request.webhook_id)
            .await?
            .or_not_found("slack_webhook", request.webhook_id)?;

        if webhook.repository_id != repository.id
            || webhook.slack_user_id != request.slack_user_id
            || webhook.slack_team_id != request.slack_team_id
            || webhook.slack_channel_id != request.slack_channel_id
        {
            return Err(NotFoundError::new("slack_webhook", request.webhook_id).into());
        }

        self.slack_webhook_repo.delete(request.webhook_id).await?;

        Ok(())
    }

    async fn list_slack_webhooks(
        &self,
        request: ListSlackWebhooksRequest,
    ) -> Result<Vec<SlackWebhookResponse>, WebhookError> {
        let owner = request.owner_name.as_ref();
        let repo = request.repo_name.as_ref();

        let repository = self
            .repo_repo
            .get(owner, repo)
            .await?
            .or_not_found("repository", format!("{owner}/{repo}"))?;

        let webhooks = self
            .slack_webhook_repo
            .list_by_repository_and_event(repository.id, request.event)
            .await?;

        Ok(webhooks.into_iter().map(Into::into).collect())
    }

    async fn notify_slack_of_repo_push(
        &self,
        request: NotifyRepoPushRequest,
    ) -> Result<(), WebhookError> {
        self.slack_bot_client
            .notify_event(WebhookEventType::Push, &request)
            .await?;
        Ok(())
    }
}
