use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

use gitdot_core::errors::{
    AuthorizationError, GitHttpBackendError, OrganizationError, RepositoryError,
};

use super::response::AppResponse;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Authorization(#[from] AuthorizationError),

    #[error(transparent)]
    Organization(#[from] OrganizationError),

    #[error(transparent)]
    Repository(#[from] RepositoryError),

    #[error(transparent)]
    GitHttpBackend(#[from] GitHttpBackendError),

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
            AppError::Authorization(e) => {
                let response = AppResponse::new(
                    StatusCode::UNAUTHORIZED,
                    AppErrorMessage {
                        message: e.to_string(),
                    },
                );
                response.into_response()
            }
            AppError::Organization(e) => {
                let status_code = match e {
                    OrganizationError::Duplicate(_) => StatusCode::CONFLICT,
                    OrganizationError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                };
                let response = AppResponse::new(
                    status_code,
                    AppErrorMessage {
                        message: e.to_string(),
                    },
                );
                response.into_response()
            }
            AppError::Repository(e) => {
                let status_code = match e {
                    RepositoryError::Duplicate(_) => StatusCode::CONFLICT,
                    RepositoryError::OwnerNotFound(_) => StatusCode::NOT_FOUND,
                    RepositoryError::GitError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                    RepositoryError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                };
                let response = AppResponse::new(
                    status_code,
                    AppErrorMessage {
                        message: e.to_string(),
                    },
                );
                response.into_response()
            }
            AppError::GitHttpBackend(e) => {
                let status_code = match &e {
                    GitHttpBackendError::InvalidService(_) => StatusCode::BAD_REQUEST,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
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
