use axum::{
    extract::{Json, State},
    http::StatusCode,
};

use gitdot_core::dto::GetQuestionsRequest;

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::{GetQuestionsServerRequest, QuestionsServerResponse};

#[axum::debug_handler]
pub async fn get_questions(
    _auth_user: Option<AuthenticatedUser>,
    State(state): State<AppState>,
    Json(request): Json<GetQuestionsServerRequest>,
) -> Result<AppResponse<QuestionsServerResponse>, AppError> {
    let request = GetQuestionsRequest::new(request.repository_id);
    state
        .question_service
        .get_questions(request)
        .await
        .map_err(AppError::from)
        .map(|qs| AppResponse::new(StatusCode::OK, qs.into()))
}
