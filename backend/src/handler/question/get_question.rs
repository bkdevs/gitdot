use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::get_question as api;
use gitdot_core::dto::{GetQuestionRequest, RepositoryAuthorizationRequest, RepositoryPermission};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::AuthenticatedUser,
};

#[axum::debug_handler]
pub async fn get_question(
    auth_user: Option<AuthenticatedUser>,
    State(state): State<AppState>,
    Path((owner, repo, number)): Path<(String, String, i32)>,
) -> Result<AppResponse<api::GetQuestionResponse>, AppError> {
    let user_id = auth_user.as_ref().map(|u| u.id);
    let request =
        RepositoryAuthorizationRequest::new(user_id, &owner, &repo, RepositoryPermission::Read)?;
    state
        .auth_service
        .verify_authorized_for_repository(request)
        .await?;

    let request = GetQuestionRequest::new(&owner, &repo, number, user_id)?;
    state
        .question_service
        .get_question(request)
        .await
        .map_err(AppError::from)
        .map(|q| AppResponse::new(StatusCode::OK, q.into_api()))
}
