mod create_repository;

use axum::{Router, routing::post};

use crate::app::AppState;

use create_repository::create_repository;

pub fn create_repository_router() -> Router<AppState> {
    Router::new().route("/repository/{owner}/{repo}", post(create_repository))
}
