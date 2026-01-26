use axum::{
    extract::{Json, State},
    http::StatusCode,
};

use gitdot_core::dto::{GetQuestionsRequest, RepositoryAuthorizationRequest};

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::{GetQuestionsServerRequest, QuestionsServerResponse};

#[axum::debug_handler]
pub async fn get_questions(
    auth_user: Option<AuthenticatedUser>,
    State(state): State<AppState>,
    Json(request): Json<GetQuestionsServerRequest>,
) -> Result<AppResponse<QuestionsServerResponse>, AppError> {
    let auth_request =
        RepositoryAuthorizationRequest::with_id(auth_user.map(|u| u.id), request.repository_id);
    state
        .auth_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let request = GetQuestionsRequest::new(request.repository_id);
    state
        .question_service
        .get_questions(request)
        .await
        .map_err(AppError::from)
        .map(|qs| AppResponse::new(StatusCode::OK, qs.into()))
}
