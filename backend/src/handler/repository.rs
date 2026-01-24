mod create_repository;
mod get_repository_file;
mod get_repository_tree;

use axum::{
    Router,
    routing::{get, post},
};

use crate::app::AppState;

use create_repository::create_repository;
use get_repository_file::get_repository_file;
use get_repository_tree::get_repository_tree;

pub fn create_repository_router() -> Router<AppState> {
    Router::new()
        .route("/repository/{owner}/{repo}", post(create_repository))
        .route("/repository/{owner}/{repo}/tree", get(get_repository_tree))
        .route("/repository/{owner}/{repo}/file", get(get_repository_file))
}
