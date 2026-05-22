use std::sync::Arc;

use axum::extract::FromRef;
use jsonwebtoken::jwk::JwkSet;
use secrecy::ExposeSecret;
use sqlx::PgPool;

use gitdot_axum::config::{AuthConfig, VercelOidcConfig};
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

    pub auth_config: AuthConfig,
    pub vercel_oidc_config: VercelOidcConfig,
}

impl AppState {
    pub async fn new(pool: PgPool, settings: Arc<Settings>) -> anyhow::Result<Self> {
        let session_repo = SessionRepositoryImpl::new(pool.clone());
        let token_repo = TokenRepositoryImpl::new(pool.clone());
        let user_repo = UserRepositoryImpl::new(pool.clone());
        let device_repo = DeviceRepositoryImpl::new(pool.clone());
        let slack_repo = SlackRepositoryImpl::new(pool.clone());

        let email_client = ResendClient::new(settings.resend_api_key.expose_secret());
        let token_client =
            TokenClientImpl::new(settings.gitdot_private_key.expose_secret().to_string());
        let slack_bot_client = SlackBotClientImpl::new(
            settings.gitdot_slack_bot_server_url.clone(),
            settings.gitdot_slack_secret.expose_secret().to_string(),
        );
        let github_client = OctocrabClient::new(
            settings.github_app_id,
            settings.github_app_slug.clone(),
            settings.github_app_private_key.expose_secret().to_string(),
            settings.github_client_id.clone(),
            settings.github_client_secret.expose_secret().to_string(),
            settings.gitdot_github_secret.expose_secret().to_string(),
        );
        let image_client = ImageClientImpl::new();
        let r2_client = R2ClientImpl::new(
            settings.cloudflare_account_id.clone(),
            settings.cloudflare_r2_bucket_name.clone(),
            settings.cloudflare_r2_access_key_id.clone(),
            settings
                .cloudflare_r2_secret_access_key
                .expose_secret()
                .to_string(),
        )
        .await;
        let redis_client = {
            let client = RedisClientImpl::new(settings.gitdot_redis_url.expose_secret()).await?;
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

        let vercel_jwks = {
            let jwks_url = format!("{}/.well-known/jwks", settings.vercel_oidc_url);
            reqwest::get(&jwks_url).await?.json::<JwkSet>().await?
        };
        let vercel_oidc_config = VercelOidcConfig {
            jwks: Arc::new(vercel_jwks),
            issuer: settings.vercel_oidc_url.clone(),
        };
        let auth_config = AuthConfig {
            public_key: settings.gitdot_public_key.clone(),
        };

        Ok(Self {
            settings,
            session_service,
            device_service,
            slack_service,
            auth_config,
            vercel_oidc_config,
        })
    }
}
