mod create_repository;
mod create_repository_commit_filter;
mod delete_repository;
mod delete_repository_commit_filter;
mod get_repository;
mod get_repository_activity;
mod get_repository_blob;
mod get_repository_blobs;
mod get_repository_commit;
mod get_repository_commit_blobs;
mod get_repository_paths;
mod get_repository_resources;
mod list_latest_repositories;
mod list_repository_commit_filters;
mod list_repository_commits;
mod list_trending_repositories;
mod star_repository;
mod unstar_repository;
mod update_repository;
mod update_repository_commit_filter;

use axum::{
    Router,
    routing::{get, patch, post},
};

use crate::app::AppState;

use create_repository::create_repository;
use create_repository_commit_filter::create_repository_commit_filter;
use delete_repository::delete_repository;
use delete_repository_commit_filter::delete_repository_commit_filter;
use get_repository::get_repository;
use get_repository_activity::get_repository_activity;
use get_repository_blob::get_repository_blob;
use get_repository_blobs::get_repository_blobs;
use get_repository_commit::get_repository_commit;
use get_repository_commit_blobs::get_repository_commit_blobs;
use get_repository_paths::get_repository_paths;
use get_repository_resources::get_repository_resources;
use list_latest_repositories::list_latest_repositories;
use list_repository_commit_filters::list_repository_commit_filters;
use list_repository_commits::list_repository_commits;
use list_trending_repositories::list_trending_repositories;
use star_repository::star_repository;
use unstar_repository::unstar_repository;
use update_repository::update_repository;
use update_repository_commit_filter::update_repository_commit_filter;

pub fn create_repository_router() -> Router<AppState> {
    Router::new()
        .route("/repository/latest", get(list_latest_repositories))
        .route("/repository/trending", get(list_trending_repositories))
        .route(
            "/repository/{owner}/{repo}",
            post(create_repository)
                .get(get_repository)
                .patch(update_repository)
                .delete(delete_repository),
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
            get(list_repository_commits),
        )
        .route(
            "/repository/{owner}/{repo}/commits/{sha}",
            get(get_repository_commit),
        )
        .route(
            "/repository/{owner}/{repo}/commits/{sha}/blobs",
            get(get_repository_commit_blobs),
        )
        .route(
            "/repository/{owner}/{repo}/resources",
            post(get_repository_resources),
        )
        .route("/repository/{owner}/{repo}/star", post(star_repository))
        .route("/repository/{owner}/{repo}/unstar", post(unstar_repository))
        .route(
            "/repository/{owner}/{repo}/activity",
            get(get_repository_activity),
        )
        .route(
            "/repository/{owner}/{repo}/commit-filters",
            post(create_repository_commit_filter).get(list_repository_commit_filters),
        )
        .route(
            "/repository/{owner}/{repo}/commit-filters/{filter_id}",
            patch(update_repository_commit_filter).delete(delete_repository_commit_filter),
        )
}
