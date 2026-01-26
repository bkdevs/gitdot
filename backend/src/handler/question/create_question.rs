use axum::{
    extract::{Json, State},
    http::StatusCode,
};

use gitdot_core::dto::CreateQuestionRequest;

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::{CreateQuestionServerRequest, QuestionServerResponse};

#[axum::debug_handler]
pub async fn create_question(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Json(request): Json<CreateQuestionServerRequest>,
) -> Result<AppResponse<QuestionServerResponse>, AppError> {
    let request = CreateQuestionRequest::new(
        auth_user.id,
        request.repository_id,
        request.title,
        request.body,
    );
    state
        .question_service
        .create_question(request)
        .await
        .map_err(AppError::from)
        .map(|q| AppResponse::new(StatusCode::CREATED, q.into()))
}
