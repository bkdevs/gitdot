use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::PgPool;

use gitdot_core::{
    client::{OctocrabClient, ResendClient, TokenClientImpl},
    repository::{
        CodeRepositoryImpl, SessionRepositoryImpl, TokenRepositoryImpl, UserRepositoryImpl,
    },
    service::{AuthenticationService, AuthenticationServiceImpl, OAuthService, OAuthServiceImpl},
};

use super::Settings;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,

    pub authentication_service: Arc<dyn AuthenticationService>,
    pub oauth_service: Arc<dyn OAuthService>,
}

impl AppState {
    pub fn new(pool: PgPool, settings: Arc<Settings>) -> Self {
        let session_repo = SessionRepositoryImpl::new(pool.clone());
        let token_repo = TokenRepositoryImpl::new(pool.clone());
        let user_repo = UserRepositoryImpl::new(pool.clone());
        let code_repo = CodeRepositoryImpl::new(pool.clone());

        let email_client = ResendClient::new(&settings.resend_api_key);
        let token_client = TokenClientImpl::new(settings.gitdot_private_key.clone());
        let github_client = OctocrabClient::new(
            settings.github_app_id,
            settings.github_app_private_key.clone(),
            settings.github_client_id.clone(),
            settings.github_client_secret.clone(),
        );

        let authentication_service = Arc::new(AuthenticationServiceImpl::new(
            session_repo.clone(),
            token_repo.clone(),
            user_repo.clone(),
            email_client,
            token_client.clone(),
        ));
        let oauth_service = Arc::new(OAuthServiceImpl::new(
            code_repo,
            session_repo,
            token_repo,
            user_repo,
            github_client,
            token_client,
        ));

        Self {
            settings,
            authentication_service,
            oauth_service,
        }
    }
}
