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
                    AuthenticationError::InvalidEmail(_) => StatusCode::BAD_REQUEST,
                    AuthenticationError::AuthCodeNotFound => StatusCode::NOT_FOUND,
                    AuthenticationError::AuthCodeAlreadyUsed => StatusCode::GONE,
                    AuthenticationError::AuthCodeExpired => StatusCode::GONE,
                    AuthenticationError::SessionNotFound => StatusCode::UNAUTHORIZED,
                    AuthenticationError::SessionExpired => StatusCode::UNAUTHORIZED,
                    AuthenticationError::SessionRevoked => StatusCode::UNAUTHORIZED,
                    AuthenticationError::MissingAuthHeader => StatusCode::UNAUTHORIZED,
                    AuthenticationError::InvalidAuthHeaderFormat => StatusCode::UNAUTHORIZED,
                    AuthenticationError::InvalidPublicKey(_) => StatusCode::INTERNAL_SERVER_ERROR,
                    AuthenticationError::InvalidToken(_) => StatusCode::UNAUTHORIZED,
                    AuthenticationError::InvalidOAuthState(_) => StatusCode::BAD_REQUEST,
                    AuthenticationError::GitHubError(_) => StatusCode::BAD_GATEWAY,
                    AuthenticationError::JwtError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                    AuthenticationError::EmailError(_) => StatusCode::BAD_GATEWAY,
                    AuthenticationError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                };
                let body = ErrorMessage {
                    message: e.to_string(),
                };
                (status_code, Json(body)).into_response()
            }
        }
    }
}
