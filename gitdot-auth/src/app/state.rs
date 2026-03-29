use axum::extract::FromRef;
use sqlx::PgPool;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub pool: PgPool,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
