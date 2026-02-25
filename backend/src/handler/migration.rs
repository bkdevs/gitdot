mod create_github_installation;
mod get_migration;
mod list_github_installation_repositories;
mod list_github_installations;
mod list_migrations;
mod migrate_github_repositories;

use axum::{
    Router,
    routing::{get, post},
};

use crate::app::AppState;

use create_github_installation::create_github_installation;
use get_migration::get_migration;
use list_github_installation_repositories::list_github_installation_repositories;
use list_github_installations::list_github_installations;
use list_migrations::list_migrations;
use migrate_github_repositories::migrate_github_repositories;

pub fn create_migration_router() -> Router<AppState> {
    Router::new()
        .route("/migrations", get(list_migrations))
        .route("/migration/{number}", get(get_migration))
        .route(
            "/migration/github/installations",
            get(list_github_installations),
        )
        .route(
            "/migration/github/{installation_id}",
            post(create_github_installation),
        )
        .route(
            "/migration/github/{installation_id}/repositories",
            get(list_github_installation_repositories),
        )
        .route(
            "/migration/github/{installation_id}/migrate",
            post(migrate_github_repositories),
        )
}
