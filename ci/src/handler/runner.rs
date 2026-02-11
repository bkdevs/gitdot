mod create_runner;
mod delete_runner;

use axum::{
    Router,
    routing::{delete, post},
};

use crate::app::AppState;

use create_runner::create_runner;
use delete_runner::delete_runner;

pub fn create_runner_router() -> Router<AppState> {
    Router::new()
        .route("/runner", post(create_runner))
        .route("/runner/:id", delete(delete_runner))
}
