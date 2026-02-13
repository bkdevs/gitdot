use crate::{
    app::{AppError, AppResponse, AppState, AuthenticatedUser},
    dto::IntoApi,
};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};

use api::endpoint::create_question_comment as api;
use gitdot_core::dto::{CreateQuestionCommentRequest, RepositoryAuthorizationRequest};

#[axum::debug_handler]
pub async fn create_question_comment(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path((owner, repo, number)): Path<(String, String, i32)>,
    Json(request): Json<api::CreateQuestionCommentRequest>,
) -> Result<AppResponse<api::CreateQuestionCommentResponse>, AppError> {
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
        .map(|c| AppResponse::new(StatusCode::CREATED, c.into_api()))
}
