use crate::{
    app::{AppError, AppResponse, AppState, AuthenticatedUser},
    dto::IntoApi,
};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use api::endpoint::create_answer_comment as api;
use gitdot_core::dto::{CreateAnswerCommentRequest, RepositoryAuthorizationRequest};

#[axum::debug_handler]
pub async fn create_answer_comment(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path((owner, repo, _number, answer_id)): Path<(String, String, i32, Uuid)>,
    Json(request): Json<api::CreateAnswerCommentRequest>,
) -> Result<AppResponse<api::CreateAnswerCommentResponse>, AppError> {
    let auth_request = RepositoryAuthorizationRequest::new(Some(auth_user.id), &owner, &repo)?;
    state
        .auth_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let request = CreateAnswerCommentRequest::new(auth_user.id, answer_id, request.body);
    state
        .question_service
        .create_answer_comment(request)
        .await
        .map_err(AppError::from)
        .map(|c| AppResponse::new(StatusCode::CREATED, c.into_api()))
}
