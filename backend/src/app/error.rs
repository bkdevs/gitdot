use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

use gitdot_core::error::{
    AuthorizationError, GitHttpError, OrganizationError, QuestionError, RepositoryError,
};

use super::AppResponse;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Authorization(#[from] AuthorizationError),

    #[error(transparent)]
    Organization(#[from] OrganizationError),

    #[error(transparent)]
    Repository(#[from] RepositoryError),

    #[error(transparent)]
    Question(#[from] QuestionError),

    #[error(transparent)]
    GitHttp(#[from] GitHttpError),

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
                let status_code = match e {
                    AuthorizationError::InvalidRequest(_) => StatusCode::BAD_REQUEST,
                    AuthorizationError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
            AppError::Organization(e) => {
                let status_code = match e {
                    OrganizationError::Duplicate(_) => StatusCode::CONFLICT,
                    OrganizationError::NotFound(_) => StatusCode::NOT_FOUND,
                    OrganizationError::InvalidOrganizationName(_) => StatusCode::BAD_REQUEST,
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
                    RepositoryError::InvalidOwnerName(_) => StatusCode::BAD_REQUEST,
                    RepositoryError::InvalidRepositoryName(_) => StatusCode::BAD_REQUEST,
                    RepositoryError::InvalidOwnerType(_) => StatusCode::BAD_REQUEST,
                    RepositoryError::InvalidVisibility(_) => StatusCode::BAD_REQUEST,
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
            AppError::Question(e) => {
                let status_code = match e {
                    QuestionError::QuestionNotFound(_) => StatusCode::NOT_FOUND,
                    QuestionError::AnswerNotFound(_) => StatusCode::NOT_FOUND,
                    QuestionError::CommentNotFound(_) => StatusCode::NOT_FOUND,
                    QuestionError::RepositoryNotFound(_) => StatusCode::NOT_FOUND,
                    QuestionError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                };
                let response = AppResponse::new(
                    status_code,
                    AppErrorMessage {
                        message: e.to_string(),
                    },
                );
                response.into_response()
            }
            AppError::GitHttp(e) => {
                let status_code = match e {
                    GitHttpError::InvalidOwnerName(_) => StatusCode::BAD_REQUEST,
                    GitHttpError::InvalidRepositoryName(_) => StatusCode::BAD_REQUEST,
                    GitHttpError::InvalidService(_) => StatusCode::BAD_REQUEST,
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
