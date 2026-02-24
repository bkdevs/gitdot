mod create_github_installation;
mod list_github_installation_repositories;

use axum::{
    Router,
    routing::{get, post},
};

use crate::app::AppState;

use create_github_installation::create_github_installation;
use list_github_installation_repositories::list_github_installation_repositories;

pub fn create_migration_router() -> Router<AppState> {
    Router::new()
        .route(
            "/migration/github/{installation_id}",
            post(create_github_installation),
        )
        .route(
            "/migration/github/{installation_id}/repositories",
            get(list_github_installation_repositories),
        )
}
