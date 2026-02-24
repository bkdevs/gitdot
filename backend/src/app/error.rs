use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use gitdot_api::ApiResource;
use gitdot_core::error::{
    AuthorizationError, BuildError, CommitError, GitHttpError, MigrationError, OrganizationError,
    QuestionError, RepositoryError, RunnerError, TaskError, TokenError, UserError,
};

use super::AppResponse;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Authorization(#[from] AuthorizationError),

    #[error(transparent)]
    Token(#[from] TokenError),

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
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppErrorMessage {
    pub message: String,
}
impl ApiResource for AppErrorMessage {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Authorization(e) => {
                let status_code = match e {
                    AuthorizationError::InvalidRequest(_) => StatusCode::BAD_REQUEST,
                    AuthorizationError::NotFound(_) => StatusCode::NOT_FOUND,
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
            AppError::Token(e) => {
                let status_code = match &e {
                    TokenError::AuthorizationPending => StatusCode::BAD_REQUEST,
                    TokenError::ExpiredToken => StatusCode::BAD_REQUEST,
                    TokenError::AccessDenied => StatusCode::BAD_REQUEST,
                    TokenError::InvalidTokenType => StatusCode::BAD_REQUEST,
                    TokenError::InvalidDeviceCode => StatusCode::BAD_REQUEST,
                    TokenError::InvalidUserCode(_) => StatusCode::BAD_REQUEST,
                    TokenError::InvalidRequest(_) => StatusCode::BAD_REQUEST,
                    TokenError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                };
                let response = AppResponse::new(
                    status_code,
                    AppErrorMessage {
                        message: e.to_string(),
                    },
                );
                response.into_response()
            }
            AppError::User(e) => {
                let status_code = match e {
                    UserError::NotFound(_) => StatusCode::NOT_FOUND,
                    UserError::InvalidUserName(_) => StatusCode::BAD_REQUEST,
                    UserError::NameTaken(_) => StatusCode::CONFLICT,
                    UserError::ReservedName(_) => StatusCode::CONFLICT,
                    UserError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
                    OrganizationError::MemberAlreadyExists(_) => StatusCode::CONFLICT,
                    OrganizationError::NotFound(_) => StatusCode::NOT_FOUND,
                    OrganizationError::UserNotFound(_) => StatusCode::NOT_FOUND,
                    OrganizationError::InvalidOrganizationName(_) => StatusCode::BAD_REQUEST,
                    OrganizationError::InvalidUserName(_) => StatusCode::BAD_REQUEST,
                    OrganizationError::InvalidRole(_) => StatusCode::BAD_REQUEST,
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
                tracing::error!("{}", e);
                let status_code = match e {
                    RepositoryError::Duplicate(_) => StatusCode::CONFLICT,
                    RepositoryError::NotFound(_) => StatusCode::NOT_FOUND,
                    RepositoryError::OwnerNotFound(_) => StatusCode::NOT_FOUND,
                    RepositoryError::InvalidOwnerName(_) => StatusCode::BAD_REQUEST,
                    RepositoryError::InvalidRepositoryName(_) => StatusCode::BAD_REQUEST,
                    RepositoryError::InvalidOwnerType(_) => StatusCode::BAD_REQUEST,
                    RepositoryError::InvalidVisibility(_) => StatusCode::BAD_REQUEST,
                    RepositoryError::GitError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                    RepositoryError::DiffError(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
            AppError::Commit(e) => {
                let status_code = match e {
                    CommitError::InvalidOwnerName(_) => StatusCode::BAD_REQUEST,
                    CommitError::InvalidRepositoryName(_) => StatusCode::BAD_REQUEST,
                    CommitError::RepositoryNotFound(_) => StatusCode::NOT_FOUND,
                    CommitError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                    CommitError::GitError(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
                    QuestionError::InvalidOwnerName(_) => StatusCode::BAD_REQUEST,
                    QuestionError::InvalidRepositoryName(_) => StatusCode::BAD_REQUEST,
                    QuestionError::InvalidVoteValue(_) => StatusCode::BAD_REQUEST,
                    QuestionError::QuestionNotFound(_) => StatusCode::NOT_FOUND,
                    QuestionError::AnswerNotFound(_) => StatusCode::NOT_FOUND,
                    QuestionError::CommentNotFound(_) => StatusCode::NOT_FOUND,
                    QuestionError::RepositoryNotFound(_) => StatusCode::NOT_FOUND,
                    QuestionError::VoteTargetNotFound(_) => StatusCode::NOT_FOUND,
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
            AppError::Migration(e) => {
                let status_code = match e {
                    MigrationError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                    MigrationError::GitHubError(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
            AppError::Build(e) => {
                let status_code = match e {
                    BuildError::InvalidOwnerName(_) => StatusCode::BAD_REQUEST,
                    BuildError::InvalidRepositoryName(_) => StatusCode::BAD_REQUEST,
                    BuildError::NotFound(_) => StatusCode::NOT_FOUND,
                    BuildError::ConfigNotFound(_) => StatusCode::NOT_FOUND,
                    BuildError::ParseError(_) => StatusCode::UNPROCESSABLE_ENTITY,
                    BuildError::GitError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                    BuildError::JoinError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                    BuildError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                };
                let response = AppResponse::new(
                    status_code,
                    AppErrorMessage {
                        message: e.to_string(),
                    },
                );
                response.into_response()
            }
            AppError::Task(e) => {
                let status_code = match e {
                    TaskError::InvalidStatus(_) => StatusCode::BAD_REQUEST,
                    TaskError::InvalidOwnerName(_) => StatusCode::BAD_REQUEST,
                    TaskError::InvalidRepositoryName(_) => StatusCode::BAD_REQUEST,
                    TaskError::NotFound(_) => StatusCode::NOT_FOUND,
                    TaskError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
