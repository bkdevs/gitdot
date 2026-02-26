mod get_migration;
mod github;
mod list_migrations;

use axum::{Router, routing::get};

use crate::app::AppState;

use get_migration::get_migration;
use github::create_github_migration_router;
use list_migrations::list_migrations;

pub fn create_migration_router() -> Router<AppState> {
    Router::new()
        .route("/migrations", get(list_migrations))
        .route("/migration/{number}", get(get_migration))
        .merge(create_github_migration_router())
}
