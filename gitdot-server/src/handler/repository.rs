mod create_repository;
mod delete_repository;
mod get_repository_blob;
mod get_repository_blobs;
mod get_repository_commit;
mod get_repository_commit_diff;
mod get_repository_commits;
mod get_repository_paths;
mod get_repository_resources;
mod get_repository_settings;
mod update_repository_settings;

use axum::{
    Router,
    routing::{get, post},
};

use crate::app::AppState;

use create_repository::create_repository;
use delete_repository::delete_repository;
use get_repository_blob::get_repository_blob;
use get_repository_blobs::get_repository_blobs;
use get_repository_commit::get_repository_commit;
use get_repository_commit_diff::get_repository_commit_diff;
use get_repository_commits::get_repository_commits;
use get_repository_paths::get_repository_paths;
use get_repository_resources::get_repository_resources;
use get_repository_settings::get_repository_settings;
use update_repository_settings::update_repository_settings;

pub fn create_repository_router() -> Router<AppState> {
    Router::new()
        .route(
            "/repository/{owner}/{repo}",
            post(create_repository).delete(delete_repository),
        )
        .route("/repository/{owner}/{repo}/blob", get(get_repository_blob))
        .route(
            "/repository/{owner}/{repo}/blobs",
            post(get_repository_blobs),
        )
        .route(
            "/repository/{owner}/{repo}/paths",
            get(get_repository_paths),
        )
        .route(
            "/repository/{owner}/{repo}/commits",
            get(get_repository_commits),
        )
        .route(
            "/repository/{owner}/{repo}/commits/{sha}",
            get(get_repository_commit),
        )
        .route(
            "/repository/{owner}/{repo}/commits/{sha}/diff",
            get(get_repository_commit_diff),
        )
        .route(
            "/repository/{owner}/{repo}/settings",
            get(get_repository_settings).patch(update_repository_settings),
        )
        .route(
            "/repository/{owner}/{repo}/resources",
            post(get_repository_resources),
        )
}
