use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::PgPool;

use gitdot_core::{
    client::{
        ImageClientImpl, OctocrabClient, R2ClientImpl, RedisClient, RedisClientImpl, ResendClient,
        SlackBotClientImpl, TokenClientImpl,
    },
    repository::{
        DeviceRepositoryImpl, SessionRepositoryImpl, SlackRepositoryImpl, TokenRepositoryImpl,
        UserRepositoryImpl,
    },
    service::{
        DeviceService, DeviceServiceImpl, SessionService, SessionServiceImpl, SlackService,
        SlackServiceImpl,
    },
};

use super::Settings;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
    pub session_service: Arc<dyn SessionService>,
    pub device_service: Arc<dyn DeviceService>,
    pub slack_service: Arc<dyn SlackService>,
}

impl AppState {
    pub async fn new(pool: PgPool, settings: Arc<Settings>) -> anyhow::Result<Self> {
        let session_repo = SessionRepositoryImpl::new(pool.clone());
        let token_repo = TokenRepositoryImpl::new(pool.clone());
        let user_repo = UserRepositoryImpl::new(pool.clone());
        let device_repo = DeviceRepositoryImpl::new(pool.clone());
        let slack_repo = SlackRepositoryImpl::new(pool.clone());

        let email_client = ResendClient::new(&settings.resend_api_key);
        let token_client = TokenClientImpl::new(settings.gitdot_private_key.clone());
        let slack_bot_client = SlackBotClientImpl::new(
            settings.gitdot_slack_bot_server_url.clone(),
            settings.gitdot_slack_secret.clone(),
        );
        let github_client = OctocrabClient::new(
            settings.github_app_id,
            settings.github_app_private_key.clone(),
            settings.github_client_id.clone(),
            settings.github_client_secret.clone(),
            settings.gitdot_github_secret.clone(),
        );
        let image_client = ImageClientImpl::new();
        let r2_client = R2ClientImpl::new(
            settings.cloudflare_account_id.clone(),
            settings.cloudflare_r2_bucket_name.clone(),
            settings.cloudflare_r2_access_key_id.clone(),
            settings.cloudflare_r2_secret_access_key.clone(),
        )
        .await;
        let redis_client = {
            let client = RedisClientImpl::new(&settings.gitdot_redis_url).await?;
            client.ping().await?;
            client
        };

        let session_service = Arc::new(SessionServiceImpl::new(
            session_repo,
            user_repo.clone(),
            email_client,
            github_client,
            token_client.clone(),
            image_client,
            r2_client,
            redis_client,
        ));
        let device_service = Arc::new(DeviceServiceImpl::new(
            device_repo,
            token_repo,
            user_repo,
            token_client,
        ));
        let slack_service = Arc::new(SlackServiceImpl::new(slack_repo, slack_bot_client));

        Ok(Self {
            settings,
            session_service,
            device_service,
            slack_service,
        })
    }
}
