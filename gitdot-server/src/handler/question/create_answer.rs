use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::create_answer as api;
use gitdot_core::dto::{CreateAnswerRequest, RepositoryAuthorizationRequest, RepositoryPermission};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn create_answer(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, number)): Path<(String, String, i32)>,
    Json(request): Json<api::CreateAnswerRequest>,
) -> Result<AppResponse<api::CreateAnswerResponse>, AppError> {
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

    let request = CreateAnswerRequest::new(auth_user.id, &owner, &repo, number, request.body)?;
    state
        .question_service
        .create_answer(request)
        .await
        .map_err(AppError::from)
        .map(|a| AppResponse::new(StatusCode::CREATED, a.into_api()))
}
