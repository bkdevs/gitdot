use std::time::{Duration, SystemTime};

use axum::{Json, http::StatusCode};
use opentelemetry::{
    global,
    trace::{SpanKind, Tracer as _},
};
use serde::Deserialize;

use crate::app::{AppError, AppResponse};

#[derive(Deserialize)]
pub struct IngestSpanRequest {
    pub url: String,
    pub start_time: u64,
    pub end_time: u64,
}

#[axum::debug_handler]
pub async fn ingest_span(
    Json(request): Json<IngestSpanRequest>,
) -> Result<AppResponse<()>, AppError> {
    let tracer = global::tracer("gitdot");
    let _span = tracer
        .span_builder(request.url)
        .with_kind(SpanKind::Client)
        .with_start_time(SystemTime::UNIX_EPOCH + Duration::from_millis(request.start_time))
        .with_end_time(SystemTime::UNIX_EPOCH + Duration::from_millis(request.end_time))
        .start(&tracer);

    Ok(AppResponse::new(StatusCode::NO_CONTENT, ()))
}
