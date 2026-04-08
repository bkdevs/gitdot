use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use gitdot_api::ApiResource;
use gitdot_core::error::{
    AuthenticationError, AuthorizationError, BuildError, CommitError, GitHttpError, MigrationError,
    OrganizationError, QuestionError, RepositoryError, ReviewError, RunnerError, TaskError,
    UserError, WebhookError,
};

use super::AppResponse;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Authentication(#[from] AuthenticationError),

    #[error(transparent)]
    Authorization(#[from] AuthorizationError),

    #[error(transparent)]
    User(#[from] UserError),

    #[error(transparent)]
    Organization(#[from] OrganizationError),

    #[error(transparent)]
    Repository(#[from] RepositoryError),

    #[error(transparent)]
    Commit(#[from] CommitError),

    #[error(transparent)]
    Question(#[from] QuestionError),

    #[error(transparent)]
    Review(#[from] ReviewError),

    #[error(transparent)]
    Migration(#[from] MigrationError),

    #[error(transparent)]
    GitHttp(#[from] GitHttpError),

    #[error(transparent)]
    Runner(#[from] RunnerError),

    #[error(transparent)]
    Build(#[from] BuildError),

    #[error(transparent)]
    Task(#[from] TaskError),

    #[error(transparent)]
    Webhook(#[from] WebhookError),

    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppErrorMessage {
    pub message: String,
}
impl ApiResource for AppErrorMessage {}

pub trait HttpStatus {
    fn status_code(&self) -> StatusCode;
}

impl HttpStatus for AuthenticationError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Input(_) | Self::TokenPending(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Extraction(_)
            | Self::TokenExpired(_)
            | Self::TokenRevoked(_)
            | Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::TokenError(_)
            | Self::EmailError(_)
            | Self::GitHubError(_)
            | Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl HttpStatus for AuthorizationError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Input(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl HttpStatus for UserError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Input(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::InvalidImage(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl HttpStatus for OrganizationError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Input(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl HttpStatus for RepositoryError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Input(_) | Self::TooManyPaths | Self::NotAFile(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::GitError(_) | Self::DiffError(_) | Self::DatabaseError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl HttpStatus for CommitError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Input(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::GitError(_) | Self::DiffError(_) | Self::DatabaseError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl HttpStatus for QuestionError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Input(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl HttpStatus for ReviewError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Input(_)
            | Self::CannotReviewOwnReview(_)
            | Self::ReviewNotPublishable(_)
            | Self::CommitsNotFound => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Conflict(_) | Self::DiffNotMergeable(_) => StatusCode::CONFLICT,
            Self::NotOrgAdmin(_) => StatusCode::FORBIDDEN,
            Self::GitError(_) | Self::DiffError(_) | Self::DatabaseError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl HttpStatus for MigrationError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Input(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::GitError(_) | Self::GitHubError(_) | Self::DatabaseError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl HttpStatus for GitHttpError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Input(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl HttpStatus for RunnerError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Input(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl HttpStatus for BuildError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Input(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::InvalidConfig(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::GitError(_) | Self::JoinError(_) | Self::DatabaseError(_) | Self::S2Error(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl HttpStatus for TaskError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Input(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::NoBuildConfig => StatusCode::UNPROCESSABLE_ENTITY,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl HttpStatus for WebhookError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Input(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

fn error_response(status_code: StatusCode, message: String) -> Response {
    AppResponse::new(status_code, AppErrorMessage { message }).into_response()
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Authentication(e) => error_response(e.status_code(), e.to_string()),
            AppError::Authorization(e) => error_response(e.status_code(), e.to_string()),
            AppError::User(e) => error_response(e.status_code(), e.to_string()),
            AppError::Organization(e) => error_response(e.status_code(), e.to_string()),
            AppError::Repository(e) => error_response(e.status_code(), e.to_string()),
            AppError::Commit(e) => error_response(e.status_code(), e.to_string()),
            AppError::Question(e) => error_response(e.status_code(), e.to_string()),
            AppError::Review(e) => error_response(e.status_code(), e.to_string()),
            AppError::Migration(e) => error_response(e.status_code(), e.to_string()),
            AppError::GitHttp(e) => error_response(e.status_code(), e.to_string()),
            AppError::Runner(e) => error_response(e.status_code(), e.to_string()),
            AppError::Build(e) => error_response(e.status_code(), e.to_string()),
            AppError::Task(e) => error_response(e.status_code(), e.to_string()),
            AppError::Webhook(e) => error_response(e.status_code(), e.to_string()),
            AppError::Internal(e) => {
                tracing::error!("{}", e);
                error_response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}
