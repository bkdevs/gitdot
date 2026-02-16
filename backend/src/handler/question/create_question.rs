use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::create_question as api;
use gitdot_core::dto::{CreateQuestionRequest, RepositoryAuthorizationRequest};

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::AuthenticatedUser,
    dto::IntoApi,
};

#[axum::debug_handler]
pub async fn create_question(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Json(request): Json<api::CreateQuestionRequest>,
) -> Result<AppResponse<api::CreateQuestionResponse>, AppError> {
    let auth_request = RepositoryAuthorizationRequest::new(Some(auth_user.id), &owner, &repo)?;
    state
        .auth_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let request =
        CreateQuestionRequest::new(auth_user.id, &owner, &repo, request.title, request.body)?;
    state
        .question_service
        .create_question(request)
        .await
        .map_err(AppError::from)
        .map(|q| AppResponse::new(StatusCode::CREATED, q.into_api()))
}
