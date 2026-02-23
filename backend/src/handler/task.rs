mod create_task;
mod get_task;
mod list_tasks;
mod poll_task;
mod update_task;

use axum::{Router, routing::{get, post}};

use crate::app::AppState;

use create_task::create_task;
use get_task::get_task;
use list_tasks::list_tasks;
use poll_task::poll_task;
use update_task::update_task;

pub fn create_task_router() -> Router<AppState> {
    Router::new()
        .route("/tasks", get(list_tasks))
        .route("/task", post(create_task))
        .route("/task/poll", get(poll_task))
        .route("/task/{id}", get(get_task).patch(update_task))
}
