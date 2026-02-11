use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

use gitdot_core::error::{DagError, RunnerError};

use super::AppResponse;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Runner(#[from] RunnerError),

    #[error(transparent)]
    Dag(#[from] DagError),

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
                    RunnerError::OwnerNotFound(_) => StatusCode::NOT_FOUND,
                    RunnerError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                };
                let response = AppResponse::new(
                    status_code,
                    AppErrorMessage {
                        message: e.to_string(),
                    },
                );
                response.into_response()
            }
            AppError::Dag(e) => {
                let status_code = match e {
                    DagError::InvalidOwnerName(_) => StatusCode::BAD_REQUEST,
                    DagError::InvalidRepositoryName(_) => StatusCode::BAD_REQUEST,
                    DagError::NotFound(_) => StatusCode::NOT_FOUND,
                    DagError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
