mod create_runner;
mod delete_runner;
mod refresh_runner_token;
mod verify_runner;

use axum::{
    Router,
    routing::{delete, post},
};

use crate::app::AppState;

use create_runner::create_runner;
use delete_runner::delete_runner;
use refresh_runner_token::refresh_runner_token;
use verify_runner::verify_runner;

pub fn create_runner_router() -> Router<AppState> {
    Router::new()
        .route("/runner", post(create_runner))
        .route("/runner/{id}", delete(delete_runner))
        .route("/runner/{id}/token", post(refresh_runner_token))
        .route("/runner/{id}/verify", post(verify_runner))
}
