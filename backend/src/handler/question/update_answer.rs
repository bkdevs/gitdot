use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use gitdot_api::endpoint::update_answer as api;
use gitdot_core::dto::{AnswerAuthorizationRequest, UpdateAnswerRequest};

use crate::{
    app::{AppError, AppResponse, AppState, AuthenticatedUser},
    dto::IntoApi,
};

#[axum::debug_handler]
pub async fn update_answer(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path((_owner, _repo, _number, answer_id)): Path<(String, String, i32, Uuid)>,
    Json(request): Json<api::UpdateAnswerRequest>,
) -> Result<AppResponse<api::UpdateAnswerResponse>, AppError> {
    let auth_request = AnswerAuthorizationRequest::new(auth_user.id, answer_id);
    state
        .auth_service
        .verify_authorized_for_answer(auth_request)
        .await?;

    let request = UpdateAnswerRequest::new(answer_id, request.body);
    state
        .question_service
        .update_answer(request)
        .await
        .map_err(AppError::from)
        .map(|a| AppResponse::new(StatusCode::OK, a.into_api()))
}
