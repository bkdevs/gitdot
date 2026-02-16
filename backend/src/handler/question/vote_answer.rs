use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use gitdot_api::endpoint::vote_answer as api;
use gitdot_core::dto::{RepositoryAuthorizationRequest, VoteAnswerRequest};

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::AuthenticatedUser,
    dto::IntoApi,
};

#[axum::debug_handler]
pub async fn vote_answer(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path((owner, repo, _number, answer_id)): Path<(String, String, i32, Uuid)>,
    Json(request): Json<api::VoteAnswerRequest>,
) -> Result<AppResponse<api::VoteAnswerResponse>, AppError> {
    let auth_request = RepositoryAuthorizationRequest::new(Some(auth_user.id), &owner, &repo)?;
    state
        .auth_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let request = VoteAnswerRequest::new(answer_id, auth_user.id, request.value)?;
    state
        .question_service
        .vote_answer(request)
        .await
        .map_err(AppError::from)
        .map(|r| AppResponse::new(StatusCode::OK, r.into_api()))
}
