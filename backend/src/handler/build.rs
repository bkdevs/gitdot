mod create_build;

use axum::{Router, routing::post};

use crate::app::AppState;

use create_build::create_build;

pub fn create_build_router() -> Router<AppState> {
    Router::new().route("/build", post(create_build))
}
