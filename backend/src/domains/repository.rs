mod dto;
mod handlers;

use axum::{Router, routing::post};

use crate::app::AppState;

pub fn get_repository_routes() -> Router<AppState> {
    Router::new().route(
        "/repository/{owner}/{repo}",
        post(handlers::create_repository::create_repository),
    )
}
