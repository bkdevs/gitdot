use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_core::dto::{GetQuestionsRequest, RepositoryAuthorizationRequest};

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::QuestionsServerResponse;

#[axum::debug_handler]
pub async fn get_questions(
    auth_user: Option<AuthenticatedUser>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
) -> Result<AppResponse<QuestionsServerResponse>, AppError> {
    let user_id = auth_user.as_ref().map(|u| u.id);
    let request = RepositoryAuthorizationRequest::new(user_id, &owner, &repo)?;
    state
        .auth_service
        .verify_authorized_for_repository(request)
        .await?;

    let request = GetQuestionsRequest::new(&owner, &repo, user_id)?;
    state
        .question_service
        .get_questions(request)
        .await
        .map_err(AppError::from)
        .map(|qs| AppResponse::new(StatusCode::OK, qs.into()))
}
