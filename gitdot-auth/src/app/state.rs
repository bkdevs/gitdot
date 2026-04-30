use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::PgPool;

use gitdot_core::{
    client::{
        ImageClientImpl, OctocrabClient, R2ClientImpl, ResendClient, SecretClient,
        SlackBotClientImpl, TokenClientImpl,
    },
    repository::{
        DeviceRepositoryImpl, SessionRepositoryImpl, SlackRepositoryImpl, TokenRepositoryImpl,
        UserRepositoryImpl,
    },
    service::{AuthenticationService, AuthenticationServiceImpl},
};

use super::Settings;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
    pub gitdot_public_key: Arc<String>,

    pub authentication_service: Arc<dyn AuthenticationService>,
}

impl AppState {
    pub async fn new(
        pool: PgPool,
        settings: Arc<Settings>,
        secret_client: impl SecretClient,
    ) -> anyhow::Result<Self> {
        let session_repo = SessionRepositoryImpl::new(pool.clone());
        let token_repo = TokenRepositoryImpl::new(pool.clone());
        let user_repo = UserRepositoryImpl::new(pool.clone());
        let device_repo = DeviceRepositoryImpl::new(pool.clone());
        let slack_repo = SlackRepositoryImpl::new(pool.clone());

        let gitdot_public_key = secret_client.get_gitdot_public_key().await?;
        let email_client = ResendClient::new(&secret_client.get_resend_api_key().await?);
        let gitdot_slack_secret = secret_client.get_gitdot_slack_secret().await?;
        let token_client = TokenClientImpl::new(
            secret_client.get_gitdot_private_key().await?,
            gitdot_slack_secret.clone(),
        );
        let slack_bot_client = SlackBotClientImpl::new(
            settings.gitdot_slack_bot_server_url.clone(),
            gitdot_slack_secret,
        );
        let github_client = OctocrabClient::new(
            secret_client.get_github_app_id().await?,
            secret_client.get_github_app_private_key().await?,
            secret_client.get_github_client_id().await?,
            secret_client.get_github_client_secret().await?,
        );
        let image_client = ImageClientImpl::new();
        let r2_client = R2ClientImpl::new(
            secret_client.get_cloudflare_account_id().await?,
            secret_client.get_cloudflare_r2_bucket_name().await?,
            secret_client.get_cloudflare_r2_access_key_id().await?,
            secret_client.get_cloudflare_r2_secret_access_key().await?,
        )
        .await;

        let authentication_service = Arc::new(AuthenticationServiceImpl::new(
            device_repo,
            session_repo,
            slack_repo,
            token_repo,
            user_repo,
            email_client,
            github_client,
            slack_bot_client,
            token_client,
            image_client,
            r2_client,
        ));

        Ok(Self {
            settings,
            gitdot_public_key: Arc::new(gitdot_public_key),
            authentication_service,
        })
    }
}
