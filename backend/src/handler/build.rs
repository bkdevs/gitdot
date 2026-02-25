mod create_build;
mod list_builds;

use axum::{
    Router,
    routing::{get, post},
};

use crate::app::AppState;

use create_build::create_build;
use list_builds::list_builds;

pub fn create_build_router() -> Router<AppState> {
    Router::new()
        .route("/builds", get(list_builds))
        .route("/build", post(create_build))
}
