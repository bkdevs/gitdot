use std::time::{Duration, SystemTime};

use axum::{
    Json,
    extract::{Extension, State},
    http::StatusCode,
};
use opentelemetry::{
    KeyValue, global,
    trace::{SpanKind, Tracer as _},
};
use serde::Deserialize;
use tower_http::request_id::RequestId;

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::{Principal, VercelOidc},
};

#[derive(Deserialize)]
pub struct CreateSpanRequest {
    pub url: String,
    pub start_time: u64,
    pub end_time: u64,
}

#[axum::debug_handler]
pub async fn create_span(
    _auth: Principal<VercelOidc>,
    State(_state): State<AppState>,
    Extension(request_id): Extension<RequestId>,
    Json(request): Json<CreateSpanRequest>,
) -> Result<AppResponse<()>, AppError> {
    let tracer = global::tracer("gitdot");
    let _span = tracer
        .span_builder(request.url)
        .with_kind(SpanKind::Client)
        .with_attributes(
            request_id
                .header_value()
                .to_str()
                .map(|id| vec![KeyValue::new("request_id", id.to_string())])
                .unwrap_or_default(),
        )
        .with_start_time(SystemTime::UNIX_EPOCH + Duration::from_millis(request.start_time))
        .with_end_time(SystemTime::UNIX_EPOCH + Duration::from_millis(request.end_time))
        .start(&tracer);

    Ok(AppResponse::new(StatusCode::NO_CONTENT, ()))
}
