mod create_build;
mod get_build;
mod list_build_tasks;
mod list_builds;

use axum::{
    Router,
    routing::{get, post},
};

use crate::app::AppState;

use create_build::create_build;
use get_build::get_build;
use list_build_tasks::list_build_tasks;
use list_builds::list_builds;

pub fn create_build_router() -> Router<AppState> {
    Router::new()
        .route("/repository/{owner}/{repo}/builds", get(list_builds))
        .route("/repository/{owner}/{repo}/build", post(create_build))
        .route("/repository/{owner}/{repo}/build/{number}", get(get_build))
        .route(
            "/repository/{owner}/{repo}/build/{number}/tasks",
            get(list_build_tasks),
        )
}
