use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use gitdot_core::dto::UpdateQuestionRequest;

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::{QuestionServerResponse, UpdateQuestionServerRequest};

#[axum::debug_handler]
pub async fn update_question(
    _auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(question_id): Path<Uuid>,
    Json(request): Json<UpdateQuestionServerRequest>,
) -> Result<AppResponse<QuestionServerResponse>, AppError> {
    let request = UpdateQuestionRequest::new(question_id, request.title, request.body);
    state
        .question_service
        .update_question(request)
        .await
        .map_err(AppError::from)
        .map(|q| AppResponse::new(StatusCode::OK, q.into()))
}
