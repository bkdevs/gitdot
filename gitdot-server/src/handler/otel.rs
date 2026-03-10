mod ingest_span;

use axum::{Router, routing::post};
use tower_http::{
    cors::CorsLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};

use crate::app::AppState;

use ingest_span::ingest_span;

pub fn create_otel_router() -> Router<AppState> {
    Router::new()
        .route("/otel/spans", post(ingest_span))
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(PropagateRequestIdLayer::x_request_id())
}
