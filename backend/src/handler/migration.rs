mod create_github_installation;

use axum::{Router, routing::post};

use crate::app::AppState;

use create_github_installation::create_github_installation;

pub fn create_migration_router() -> Router<AppState> {
    Router::new().route(
        "/migration/github/{installation_id}",
        post(create_github_installation),
    )
}
