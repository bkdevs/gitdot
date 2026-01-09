use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

use super::response::AppResponse;

#[derive(Debug, Error)]
pub enum AppError {
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
