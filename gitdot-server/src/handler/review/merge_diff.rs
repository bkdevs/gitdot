use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::merge_diff as api;
use gitdot_core::dto::{MergeDiffRequest, ReviewAuthorizationRequest};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn merge_diff(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, number, position)): Path<(String, String, i32, i32)>,
) -> Result<AppResponse<api::MergeDiffResponse>, AppError> {
    let auth_request = ReviewAuthorizationRequest::new(auth_user.id, &owner, &repo, number)?;
    state
        .authorization_service
        .verify_authorized_for_review(auth_request)
        .await?;

    let request = MergeDiffRequest::new(&owner, &repo, number, position)?;

    state
        .review_service
        .merge_diff(request)
        .await
        .map_err(AppError::from)
        .map(|response| AppResponse::new(StatusCode::OK, response.into_api()))
}
