mod create_build;
mod get_build_by_number;
mod list_build_tasks;
mod list_builds;

use axum::{
    Router,
    routing::{get, post},
};

use crate::app::AppState;

use create_build::create_build;
use get_build_by_number::get_build_by_number;
use list_build_tasks::list_build_tasks;
use list_builds::list_builds;

pub fn create_build_router() -> Router<AppState> {
    Router::new()
        .route("/builds", get(list_builds))
        .route("/builds/{owner}/{repo}/{number}", get(get_build_by_number))
        .route("/build", post(create_build))
        .route("/build/{id}/tasks", get(list_build_tasks))
}
