use axum::{
    extract::{Json, State},
    http::StatusCode,
};

use gitdot_core::dto::CreateAnswerRequest;

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::{AnswerServerResponse, CreateAnswerServerRequest};

#[axum::debug_handler]
pub async fn create_answer(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Json(request): Json<CreateAnswerServerRequest>,
) -> Result<AppResponse<AnswerServerResponse>, AppError> {
    let request = CreateAnswerRequest::new(auth_user.id, request.question_id, request.body);
    state
        .question_service
        .create_answer(request)
        .await
        .map_err(AppError::from)
        .map(|a| AppResponse::new(StatusCode::CREATED, a.into()))
}
