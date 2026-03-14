use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::Deserialize;

use gitdot_core::dto::{
    GetReviewDiffRequest, RepositoryAuthorizationRequest, RepositoryPermission,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[derive(Debug, Deserialize)]
pub struct GetReviewDiffQuery {
    pub revision: Option<i32>,
    pub compare_to: Option<i32>,
}

#[axum::debug_handler]
pub async fn get_review_diff(
    auth_user: Option<Principal<User>>,
    State(state): State<AppState>,
    Path((owner, repo, number, position)): Path<(String, String, i32, i32)>,
    Query(query): Query<GetReviewDiffQuery>,
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
        number,
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
