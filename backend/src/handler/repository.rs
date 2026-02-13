mod create_repository;
mod get_repository_commit;
mod get_repository_commits;
mod get_repository_file;
mod get_repository_file_commits;
mod get_repository_preview;
mod get_repository_tree;

use axum::{
    Router,
    routing::{get, post},
};

use crate::app::AppState;

use create_repository::create_repository;
use get_repository_commit::get_repository_commit;
use get_repository_commits::get_repository_commits;
use get_repository_file::get_repository_file;
use get_repository_file_commits::get_repository_file_commits;
use get_repository_preview::get_repository_preview;
use get_repository_tree::get_repository_tree;

pub fn create_repository_router() -> Router<AppState> {
    Router::new()
        .route("/repository/{owner}/{repo}", post(create_repository))
        .route("/repository/{owner}/{repo}/tree", get(get_repository_tree))
        .route(
            "/repository/{owner}/{repo}/preview",
            get(get_repository_preview),
        )
        .route("/repository/{owner}/{repo}/file", get(get_repository_file))
        .route(
            "/repository/{owner}/{repo}/commits",
            get(get_repository_commits),
        )
        .route(
            "/repository/{owner}/{repo}/commits/{sha}",
            get(get_repository_commit),
        )
        .route(
            "/repository/{owner}/{repo}/file/commits",
            get(get_repository_file_commits),
        )
}
