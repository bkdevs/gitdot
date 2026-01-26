use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use gitdot_core::dto::{RepositoryAuthorizationRequest, VoteAnswerRequest};

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::{VoteServerRequest, VoteServerResponse};

#[axum::debug_handler]
pub async fn vote_answer(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path((owner, repo, _number, answer_id)): Path<(String, String, i32, Uuid)>,
    Json(request): Json<VoteServerRequest>,
) -> Result<AppResponse<VoteServerResponse>, AppError> {
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
        .map(|r| AppResponse::new(StatusCode::OK, r.into()))
}
