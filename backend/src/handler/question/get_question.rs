use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use gitdot_core::dto::GetQuestionRequest;

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::QuestionServerResponse;

#[axum::debug_handler]
pub async fn get_question(
    _auth_user: Option<AuthenticatedUser>,
    State(state): State<AppState>,
    Path(question_id): Path<Uuid>,
) -> Result<AppResponse<QuestionServerResponse>, AppError> {
    let request = GetQuestionRequest::new(question_id);
    state
        .question_service
        .get_question(request)
        .await
        .map_err(AppError::from)
        .map(|q| AppResponse::new(StatusCode::OK, q.into()))
}
