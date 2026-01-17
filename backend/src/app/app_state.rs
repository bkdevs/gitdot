use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::PgPool;

use gitdot_core::services::{OrganizationService, OrganizationServiceImpl};

use super::settings::Settings;

#[derive(FromRef, Clone)]
pub struct AppState {
    settings: Arc<Settings>,
    org_service: Arc<dyn OrganizationService>,
}

impl AppState {
    pub fn new(settings: Arc<Settings>, pool: PgPool) -> Self {
        let org_service = Arc::new(OrganizationServiceImpl::new(pool.clone()));
        Self {
            settings,
            org_service,
        }
    }
}
