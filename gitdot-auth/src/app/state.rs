use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::PgPool;

use gitdot_core::{
    client::{ResendClient, TokenClientImpl},
    repository::{SessionRepositoryImpl, TokenRepositoryImpl, UserRepositoryImpl},
    service::{AuthenticationService, AuthenticationServiceImpl},
};

use super::Settings;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub authentication_service: Arc<dyn AuthenticationService>,
}

impl AppState {
    pub fn new(pool: PgPool, settings: &Settings) -> Self {
        let session_repo = SessionRepositoryImpl::new(pool.clone());
        let token_repo = TokenRepositoryImpl::new(pool.clone());
        let user_repo = UserRepositoryImpl::new(pool.clone());

        let email_client = ResendClient::new(&settings.resend_api_key);
        let token_client = TokenClientImpl::new();

        let authentication_service = Arc::new(AuthenticationServiceImpl::new(
            session_repo,
            token_repo,
            user_repo,
            email_client,
            token_client,
            settings.gitdot_private_key.clone(),
        ));

        Self {
            authentication_service,
        }
    }
}
