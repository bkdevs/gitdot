use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::get_review as api;
use gitdot_core::dto::{GetReviewRequest, RepositoryAuthorizationRequest, RepositoryPermission};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

use super::ReviewIdParam;

#[axum::debug_handler]
pub async fn get_review(
    auth_user: Option<Principal<User>>,
    State(state): State<AppState>,
    Path((owner, repo, review)): Path<(String, String, ReviewIdParam)>,
) -> Result<AppResponse<api::GetReviewResponse>, AppError> {
    let user_id = auth_user.as_ref().map(|u| u.id);
    let auth_request =
        RepositoryAuthorizationRequest::new(user_id, &owner, &repo, RepositoryPermission::Read)?;
    state
        .authorization_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let request = GetReviewRequest::new(&owner, &repo, review.0)?;
    state
        .review_service
        .get_review(request)
        .await
        .map_err(AppError::from)
        .map(|r| AppResponse::new(StatusCode::OK, r.into_api()))
}
