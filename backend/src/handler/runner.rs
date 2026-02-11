mod create_runner;
mod delete_runner;
mod register_runner;

use axum::{
    Router,
    routing::{delete, post},
};

use crate::app::AppState;

use create_runner::create_runner;
use delete_runner::delete_runner;
use register_runner::register_runner;

pub fn create_runner_router() -> Router<AppState> {
    Router::new()
        .route("/runner", post(create_runner))
        .route("/runner/{id}", delete(delete_runner))
        .route("/runner/{id}/register", post(register_runner))
}
