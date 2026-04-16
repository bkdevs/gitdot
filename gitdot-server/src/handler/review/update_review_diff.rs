use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::update_review_diff as api;
use gitdot_core::dto::{ReviewAuthorizationRequest, UpdateReviewDiffRequest};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

use super::ReviewIdParam;

#[axum::debug_handler]
pub async fn update_review_diff(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, id, position)): Path<(String, String, ReviewIdParam, i32)>,
    Json(request): Json<api::UpdateReviewDiffRequest>,
) -> Result<AppResponse<api::UpdateReviewDiffResponse>, AppError> {
    let auth_request = ReviewAuthorizationRequest::new(auth_user.id, &owner, &repo, id.0.clone())?;
    state
        .authorization_service
        .verify_authorized_for_review(auth_request)
        .await?;

    let request = UpdateReviewDiffRequest::new(&owner, &repo, id.0, position, request.message)?;
    state
        .review_service
        .update_review_diff(request)
        .await
        .map_err(AppError::from)
        .map(|r| AppResponse::new(StatusCode::OK, r.into_api()))
}
