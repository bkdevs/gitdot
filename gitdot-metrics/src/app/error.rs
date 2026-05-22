use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

use gitdot_core::error::MetricsError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Metrics(#[from] MetricsError),

    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, Serialize)]
struct ErrorMessage {
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::Metrics(MetricsError::Input(_)) => StatusCode::BAD_REQUEST,
            AppError::Metrics(MetricsError::ClickHouse(_)) | AppError::Internal(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        let body = ErrorMessage {
            message: self.to_string(),
        };
        (status, Json(body)).into_response()
    }
}
