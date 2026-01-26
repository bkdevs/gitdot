use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};

use gitdot_core::dto::{CreateQuestionCommentRequest, RepositoryAuthorizationRequest};

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::{CommentServerResponse, CreateCommentServerRequest};

#[axum::debug_handler]
pub async fn create_question_comment(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path((owner, repo, number)): Path<(String, String, i32)>,
    Json(request): Json<CreateCommentServerRequest>,
) -> Result<AppResponse<CommentServerResponse>, AppError> {
    let auth_request = RepositoryAuthorizationRequest::new(Some(auth_user.id), &owner, &repo)?;
    state
        .auth_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let request =
        CreateQuestionCommentRequest::new(auth_user.id, &owner, &repo, number, request.body)?;
    state
        .question_service
        .create_question_comment(request)
        .await
        .map_err(AppError::from)
        .map(|c| AppResponse::new(StatusCode::CREATED, c.into()))
}
