use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::get_questions as api;
use gitdot_core::dto::{
    ListQuestionsRequest, RepositoryAuthorizationRequest, RepositoryPermission,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::AuthenticatedUser,
};

#[axum::debug_handler]
pub async fn get_questions(
    auth_user: Option<AuthenticatedUser>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
) -> Result<AppResponse<api::GetQuestionsResponse>, AppError> {
    let user_id = auth_user.as_ref().map(|u| u.id);
    let request =
        RepositoryAuthorizationRequest::new(user_id, &owner, &repo, RepositoryPermission::Read)?;
    state
        .auth_service
        .verify_authorized_for_repository(request)
        .await?;

    let request = ListQuestionsRequest::new(&owner, &repo, user_id)?;
    state
        .question_service
        .list_questions(request)
        .await
        .map_err(AppError::from)
        .map(|qs| AppResponse::new(StatusCode::OK, qs.into_api()))
}
