use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use chrono::Utc;

use gitdot_api::endpoint::list_questions as api;
use gitdot_core::dto::{
    ListQuestionsRequest, RepositoryAuthorizationRequest, RepositoryPermission,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn list_questions(
    auth_user: Option<Principal<User>>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
) -> Result<AppResponse<api::ListQuestionsResponse>, AppError> {
    let user_id = auth_user.as_ref().map(|u| u.id);
    let request =
        RepositoryAuthorizationRequest::new(user_id, &owner, &repo, RepositoryPermission::Read)?;
    state
        .authorization_service
        .verify_authorized_for_repository(request)
        .await?;

    let now = Utc::now();
    let request = ListQuestionsRequest::new(
        &owner,
        &repo,
        user_id,
        now - chrono::Duration::weeks(2),
        now,
    )?;
    state
        .question_service
        .list_questions(request)
        .await
        .map_err(AppError::from)
        .map(|qs| AppResponse::new(StatusCode::OK, qs.questions.into_api()))
}
