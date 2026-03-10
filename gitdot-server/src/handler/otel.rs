mod create_span;

use crate::app::AppState;
use axum::{Router, routing::post};

use create_span::create_span;

pub fn create_otel_router() -> Router<AppState> {
    Router::new().route("/otel/spans", post(create_span))
}
