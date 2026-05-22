use std::sync::Arc;

use axum::extract::FromRef;
use jsonwebtoken::jwk::JwkSet;
use secrecy::ExposeSecret;

use gitdot_axum::config::{AuthConfig, VercelOidcConfig};
use gitdot_core::{
    client::{ClickHouseClient, ClickHouseClientImpl},
    service::{MetricsService, MetricsServiceImpl},
};

use super::Settings;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,

    pub metrics_service: Arc<dyn MetricsService>,

    pub auth_config: AuthConfig,
    pub vercel_oidc_config: VercelOidcConfig,
}

impl AppState {
    pub async fn new(settings: Arc<Settings>) -> anyhow::Result<Self> {
        let clickhouse = {
            let clickhouse = ClickHouseClientImpl::new(
                &settings.clickhouse_url,
                &settings.clickhouse_user,
                settings.clickhouse_password.expose_secret(),
                &settings.clickhouse_database,
            );
            clickhouse.ping().await?;
            clickhouse
        };
        let metrics_service: Arc<dyn MetricsService> =
            Arc::new(MetricsServiceImpl::new(clickhouse));

        let auth_config = AuthConfig {
            public_key: settings.gitdot_public_key.clone(),
        };
        let vercel_jwks = {
            let jwks_url = format!("{}/.well-known/jwks", settings.vercel_oidc_url);
            reqwest::get(&jwks_url).await?.json::<JwkSet>().await?
        };
        let vercel_oidc_config = VercelOidcConfig {
            jwks: Arc::new(vercel_jwks),
            issuer: settings.vercel_oidc_url.clone(),
        };

        Ok(Self {
            settings,
            metrics_service,
            auth_config,
            vercel_oidc_config,
        })
    }
}
