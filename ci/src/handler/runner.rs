mod create_runner;

use axum::{Router, routing::post};

use crate::app::AppState;

use create_runner::create_runner;

pub fn create_runner_router() -> Router<AppState> {
    Router::new().route("/runner", post(create_runner))
}
