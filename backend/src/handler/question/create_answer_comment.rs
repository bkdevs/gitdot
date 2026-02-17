use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use gitdot_api::endpoint::create_answer_comment as api;
use gitdot_core::dto::{
    CreateAnswerCommentRequest, RepositoryAuthorizationRequest, RepositoryPermission,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn create_answer_comment(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, _number, answer_id)): Path<(String, String, i32, Uuid)>,
    Json(request): Json<api::CreateAnswerCommentRequest>,
) -> Result<AppResponse<api::CreateAnswerCommentResponse>, AppError> {
    let auth_request = RepositoryAuthorizationRequest::new(
        Some(auth_user.id),
        &owner,
        &repo,
        RepositoryPermission::Read,
    )?;
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
