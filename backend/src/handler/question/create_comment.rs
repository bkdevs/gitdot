use axum::{
    extract::{Json, State},
    http::StatusCode,
};

use gitdot_core::dto::{CreateCommentRequest, RepositoryAuthorizationRequest};

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::{CommentServerResponse, CreateCommentServerRequest};

#[axum::debug_handler]
pub async fn create_comment(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Json(request): Json<CreateCommentServerRequest>,
) -> Result<AppResponse<CommentServerResponse>, AppError> {
    let auth_request =
        RepositoryAuthorizationRequest::with_id(Some(auth_user.id), request.repository_id);
    state
        .auth_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let request = CreateCommentRequest::new(auth_user.id, request.parent_id, request.body);
    state
        .question_service
        .create_comment(request)
        .await
        .map_err(AppError::from)
        .map(|c| AppResponse::new(StatusCode::CREATED, c.into()))
}
