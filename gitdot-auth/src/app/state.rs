use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::PgPool;

use gitdot_core::{
    client::{
        ImageClientImpl, OctocrabClient, R2ClientImpl, ResendClient, SlackBotClientImpl,
        TokenClientImpl,
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
        );
        let image_client = ImageClientImpl::new();
        let r2_client = R2ClientImpl::new(
            settings.cloudflare_account_id.clone(),
            settings.cloudflare_r2_bucket_name.clone(),
            settings.cloudflare_r2_access_key_id.clone(),
            settings.cloudflare_r2_secret_access_key.clone(),
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

        let gitdot_public_key = Arc::new(settings.gitdot_public_key.clone());

        Ok(Self {
            settings,
            gitdot_public_key,
            authentication_service,
        })
    }
}
