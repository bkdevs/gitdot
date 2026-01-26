use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use gitdot_core::dto::UpdateAnswerRequest;

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::{AnswerServerResponse, UpdateAnswerServerRequest};

#[axum::debug_handler]
pub async fn update_answer(
    _auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(answer_id): Path<Uuid>,
    Json(request): Json<UpdateAnswerServerRequest>,
) -> Result<AppResponse<AnswerServerResponse>, AppError> {
    let request = UpdateAnswerRequest::new(answer_id, request.body);
    state
        .question_service
        .update_answer(request)
        .await
        .map_err(AppError::from)
        .map(|a| AppResponse::new(StatusCode::OK, a.into()))
}
