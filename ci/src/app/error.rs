use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

use gitdot_core::error::RunnerError;

use super::AppResponse;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Runner(#[from] RunnerError),

    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AppErrorMessage {
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Runner(e) => {
                let status_code = match e {
                    RunnerError::InvalidRunnerName(_) => StatusCode::BAD_REQUEST,
                    RunnerError::InvalidOwnerName(_) => StatusCode::BAD_REQUEST,
                    RunnerError::InvalidOwnerType(_) => StatusCode::BAD_REQUEST,
                    RunnerError::NotFound(_) => StatusCode::NOT_FOUND,
                    RunnerError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                    _ => StatusCode::UNAUTHORIZED,
                };
                let response = AppResponse::new(
                    status_code,
                    AppErrorMessage {
                        message: e.to_string(),
                    },
                );
                response.into_response()
            }
            AppError::Internal(e) => {
                tracing::error!("{}", e);
                let response = AppResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    AppErrorMessage {
                        message: e.to_string(),
                    },
                );
                response.into_response()
            }
        }
    }
}
