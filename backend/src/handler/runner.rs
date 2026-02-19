mod create_runner;
mod delete_runner;
mod get_runner;
mod refresh_runner_token;
mod verify_runner;

use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::app::AppState;

use create_runner::create_runner;
use delete_runner::delete_runner;
use get_runner::get_runner;
use refresh_runner_token::refresh_runner_token;
use verify_runner::verify_runner;

pub fn create_runner_router() -> Router<AppState> {
    Router::new()
        .route("/runner", post(create_runner))
        .route("/runner/verify", post(verify_runner))
        .route("/runner/{owner}/{name}", get(get_runner))
        .route("/runner/{owner}/{name}", delete(delete_runner))
        .route("/runner/{owner}/{name}/token", post(refresh_runner_token))
}
