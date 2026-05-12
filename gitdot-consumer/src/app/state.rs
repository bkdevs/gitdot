use std::sync::Arc;

use anyhow::Context;
use rdkafka::{ClientConfig, consumer::StreamConsumer};
use sqlx::PgPool;

use gitdot_core::{
    client::{GcpKafkaContext, KafkaAuthMode, SlackBotClientImpl},
    repository::{RepositoryRepositoryImpl, SlackWebhookRepositoryImpl},
    service::{SlackWebhookService, SlackWebhookServiceImpl},
};

use super::Settings;

#[derive(Clone)]
pub struct ConsumerState {
    pub settings: Settings,
    pub slack_webhook_service: Arc<dyn SlackWebhookService>,
}

impl ConsumerState {
    pub async fn new(settings: Settings, pool: PgPool) -> anyhow::Result<Self> {
        let slack_webhook_repo = SlackWebhookRepositoryImpl::new(pool.clone());
        let repo_repo = RepositoryRepositoryImpl::new(pool.clone());

        let slack_bot_client = SlackBotClientImpl::new(
            settings.gitdot_slack_bot_server_url.clone(),
            settings.gitdot_slack_secret.clone(),
        );

        let slack_webhook_service = Arc::new(SlackWebhookServiceImpl::new(
            slack_webhook_repo,
            repo_repo,
            slack_bot_client,
        ));

        Ok(Self {
            settings,
            slack_webhook_service,
        })
    }
}

pub enum ConsumerHandle {
    Plain(StreamConsumer),
    Gcp(StreamConsumer<GcpKafkaContext>),
}

pub async fn build_consumer(settings: &Settings) -> anyhow::Result<ConsumerHandle> {
    let mut config = ClientConfig::new();
    config
        .set("bootstrap.servers", &settings.kafka_bootstrap_servers)
        .set("group.id", &settings.kafka_consumer_group_id)
        .set("enable.auto.commit", "false")
        .set("auto.offset.reset", "earliest")
        .set("session.timeout.ms", "10000");

    settings.kafka_auth.apply_security_config(&mut config);

    let handle = match settings.kafka_auth {
        KafkaAuthMode::GcpOauthbearer => {
            let context = GcpKafkaContext::new().await?;
            ConsumerHandle::Gcp(
                config
                    .create_with_context(context)
                    .context("create kafka consumer")?,
            )
        }
        KafkaAuthMode::Local => {
            ConsumerHandle::Plain(config.create().context("create kafka consumer")?)
        }
    };
    Ok(handle)
}
