use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};

use gitdot_api::endpoint::get_review_diff as api;
use gitdot_core::dto::{
    GetReviewDiffRequest, RepositoryAuthorizationRequest, RepositoryPermission,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

use super::ReviewIdParam;

#[axum::debug_handler]
pub async fn get_review_diff(
    auth_user: Option<Principal<User>>,
    State(state): State<AppState>,
    Path((owner, repo, id, position)): Path<(String, String, ReviewIdParam, i32)>,
    Query(query): Query<api::GetReviewDiffRequest>,
) -> Result<
    AppResponse<gitdot_api::endpoint::review::get_review_diff::GetReviewDiffResponse>,
    AppError,
> {
    let user_id = auth_user.as_ref().map(|u| u.id);
    let auth_request =
        RepositoryAuthorizationRequest::new(user_id, &owner, &repo, RepositoryPermission::Read)?;
    state
        .authorization_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let request = GetReviewDiffRequest::new(
        &owner,
        &repo,
        id.0,
        position,
        query.revision,
        query.compare_to,
    )?;
    state
        .review_service
        .get_review_diff(request)
        .await
        .map_err(AppError::from)
        .map(|response| AppResponse::new(StatusCode::OK, response.into_api()))
}
