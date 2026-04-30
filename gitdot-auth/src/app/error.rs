use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

use gitdot_core::error::AuthenticationError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Authentication(#[from] AuthenticationError),
}

#[derive(Debug, Serialize)]
struct ErrorMessage {
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Authentication(e) => {
                let status_code = match &e {
                    AuthenticationError::Input(_) | AuthenticationError::TokenPending(_) => {
                        StatusCode::BAD_REQUEST
                    }
                    AuthenticationError::NotFound(_) => StatusCode::NOT_FOUND,
                    AuthenticationError::Extraction(_)
                    | AuthenticationError::TokenExpired(_)
                    | AuthenticationError::TokenRevoked(_)
                    | AuthenticationError::Unauthorized => StatusCode::UNAUTHORIZED,
                    AuthenticationError::TokenError(_)
                    | AuthenticationError::GitHubError(_)
                    | AuthenticationError::SlackBotError(_)
                    | AuthenticationError::EmailError(_)
                    | AuthenticationError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                };
                let body = ErrorMessage {
                    message: e.to_string(),
                };
                (status_code, Json(body)).into_response()
            }
        }
    }
}
