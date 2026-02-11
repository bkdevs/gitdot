mod create_dag;

use axum::{Router, routing::post};

use crate::app::AppState;

use create_dag::create_dag;

pub fn create_dag_router() -> Router<AppState> {
    Router::new().route("/dag", post(create_dag))
}
