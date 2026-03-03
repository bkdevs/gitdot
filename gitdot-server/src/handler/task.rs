mod get_task_logs;
mod poll_task;
mod update_task;

use axum::{
    Router,
    routing::{get, patch},
};

use crate::app::AppState;

use get_task_logs::get_task_logs;
use poll_task::poll_task;
use update_task::update_task;

pub fn create_task_router() -> Router<AppState> {
    Router::new()
        .route("/task/poll", get(poll_task))
        .route("/task/{id}", patch(update_task))
        .route("/task/{id}/logs", get(get_task_logs))
}
