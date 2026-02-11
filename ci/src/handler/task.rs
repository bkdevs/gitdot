mod poll_task;
mod update_task;

use axum::{Router, routing::{get, patch}};

use crate::app::AppState;

use poll_task::poll_task;
use update_task::update_task;

pub fn create_task_router() -> Router<AppState> {
    Router::new()
        .route("/task/poll", get(poll_task))
        .route("/task/:id", patch(update_task))
}
