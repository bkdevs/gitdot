use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::vote_question as api;
use gitdot_core::dto::{RepositoryAuthorizationRequest, VoteQuestionRequest};

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::AuthenticatedUser,
    dto::IntoApi,
};

#[axum::debug_handler]
pub async fn vote_question(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path((owner, repo, number)): Path<(String, String, i32)>,
    Json(request): Json<api::VoteQuestionRequest>,
) -> Result<AppResponse<api::VoteQuestionResponse>, AppError> {
    let auth_request = RepositoryAuthorizationRequest::new(Some(auth_user.id), &owner, &repo)?;
    state
        .auth_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let request = VoteQuestionRequest::new(&owner, &repo, number, auth_user.id, request.value)?;
    state
        .question_service
        .vote_question(request)
        .await
        .map_err(AppError::from)
        .map(|r| AppResponse::new(StatusCode::OK, r.into_api()))
}
