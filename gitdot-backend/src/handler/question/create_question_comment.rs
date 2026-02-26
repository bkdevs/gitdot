use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::create_question_comment as api;
use gitdot_core::dto::{
    CreateQuestionCommentRequest, RepositoryAuthorizationRequest, RepositoryPermission,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn create_question_comment(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, number)): Path<(String, String, i32)>,
    Json(request): Json<api::CreateQuestionCommentRequest>,
) -> Result<AppResponse<api::CreateQuestionCommentResponse>, AppError> {
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

    let request =
        CreateQuestionCommentRequest::new(auth_user.id, &owner, &repo, number, request.body)?;
    state
        .question_service
        .create_question_comment(request)
        .await
        .map_err(AppError::from)
        .map(|c| AppResponse::new(StatusCode::CREATED, c.into_api()))
}
