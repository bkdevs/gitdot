use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::PgPool;

use super::Settings;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
}

impl AppState {
    pub fn new(settings: Arc<Settings>, pool: PgPool) -> Self {
        Self { settings }
    }
}
