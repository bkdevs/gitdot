use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::update_question as api;
use gitdot_core::dto::{QuestionAuthorizationRequest, UpdateQuestionRequest};

use crate::{
    app::{AppError, AppResponse, AppState, AuthenticatedUser},
    dto::IntoApi,
};

#[axum::debug_handler]
pub async fn update_question(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path((owner, repo, number)): Path<(String, String, i32)>,
    Json(request): Json<api::UpdateQuestionRequest>,
) -> Result<AppResponse<api::UpdateQuestionResponse>, AppError> {
    let auth_request = QuestionAuthorizationRequest::new(auth_user.id, &owner, &repo, number)?;
    state
        .auth_service
        .verify_authorized_for_question(auth_request)
        .await?;

    let request = UpdateQuestionRequest::new(&owner, &repo, number, request.title, request.body)?;
    state
        .question_service
        .update_question(request)
        .await
        .map_err(AppError::from)
        .map(|q| AppResponse::new(StatusCode::OK, q.into_api()))
}
