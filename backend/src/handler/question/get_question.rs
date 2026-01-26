use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_core::dto::{GetQuestionRequest, RepositoryAuthorizationRequest};

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::QuestionServerResponse;

#[axum::debug_handler]
pub async fn get_question(
    auth_user: Option<AuthenticatedUser>,
    State(state): State<AppState>,
    Path((owner, repo, number)): Path<(String, String, i32)>,
) -> Result<AppResponse<QuestionServerResponse>, AppError> {
    let request = RepositoryAuthorizationRequest::new(auth_user.map(|u| u.id), &owner, &repo)?;
    state
        .auth_service
        .verify_authorized_for_repository(request)
        .await?;

    let request = GetQuestionRequest::new(&owner, &repo, number)?;
    state
        .question_service
        .get_question(request)
        .await
        .map_err(AppError::from)
        .map(|q| AppResponse::new(StatusCode::OK, q.into()))
}
